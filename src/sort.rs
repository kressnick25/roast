use colored::*;
use regex::Regex;
use serde::ser::Serialize;
use serde_json::{Serializer, Value};
use std::env;
use std::error::Error;
use std::fmt::Display;
use std::fs::{self};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub use crate::formatter::LineFormatter;
pub use crate::lines::LineEnding;

const INVALID_PATH: &'static str = "INVALID_PATH";
const IGNORED_FILES: &'static [&str] = &[
    "node_modules",
    "package.json",
    "package_lock.json",
    ".DS_Store",
    "npm-debug.log",
    ".svn",
    "CVS",
    "config.gypi",
    ".lock-wscript",
    "package-lock.json",
    "npm-shrinkwrap.json",
];

/// Reason why a [Path] could not be JSON sorted
#[derive(Debug)]
pub enum JsonError {
    NotFound,
    ReadError,
    ParseError,
    WriteError,
}

/// Result of a sort operation for a JSON file
///
///  * `path` - [Path] of the file that was sorted
///  * `error` - [JsonError] if the sort operation failed
///
pub struct SortResult {
    path: Box<Path>,
    error: Option<JsonError>,
}

impl SortResult {
    pub fn success(&self) -> bool {
        self.error.is_none()
    }
}

impl<'a> Display for SortResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path_str = path_to_relative(&self.path).unwrap_or(INVALID_PATH.into());

        if self.success() {
            write!(f, "{} - {}", path_str, "OK".green().bold())
        } else {
            let err_msg = format!("{:?}", self.error.as_ref().expect("Not possible"))
                .red()
                .bold();
            write!(f, "{} - {}", path_str, err_msg)
        }
    }
}

/// Read a list of JSON files, sort the contents of each and save the output to disk.
///
/// ## Arguments
///
/// * `files` - a list of relative or absolute Paths to sort
/// * `line_ending` - type of line ending/seperator to use for newlines
/// * `use_spaces` - use _spaces_ for whitespace, instead of default _tabs_
/// * `sort_arrays` - enable to sort arrays. Only sorts arrays containing all string types
/// * `indents` - number of whitespace indents to use
/// * `dry_run` - print files that would be sorted, but do not modify
///
/// Ignores files that should not be modified. See [IGNORED_FILES]
///
#[inline]
pub fn sort_files(
    files: &Vec<PathBuf>,
    line_ending: &LineEnding,
    use_spaces: bool,
    sort_arrays: bool,
    indents: usize,
    dry_run: bool,
) -> Vec<SortResult> {
    let mut results: Vec<SortResult> = vec![];
    for path in files {
        if path.is_dir() {
            for entry in WalkDir::new(path)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file())
            {
                let entry_path = entry.path().to_path_buf();
                match sort_path(
                    &entry_path,
                    dry_run,
                    line_ending,
                    use_spaces,
                    sort_arrays,
                    indents,
                    &results,
                ) {
                    Some(res) => results.push(res),
                    _ => (),
                }
            }
        } else {
            match sort_path(
                path,
                dry_run,
                line_ending,
                use_spaces,
                sort_arrays,
                indents,
                &results,
            ) {
                Some(res) => results.push(res),
                _ => (),
            }
        }
    }
    results
}

fn sort_path(
    path: &PathBuf,
    dry_run: bool,
    line_ending: &LineEnding,
    use_spaces: bool,
    sort_arrays: bool,
    indents: usize,
    results: &Vec<SortResult>,
) -> Option<SortResult> {
    if !path.exists() {
        return Some(SortResult {
            path: path.as_path().into(),
            error: Some(JsonError::NotFound),
        });
    }
    if is_ignored(&path) || is_already_sorted(&path, &results) {
        log::debug!("Ignored: {:?}", path.to_str());
        return None;
    }

    let file: String = match read_file(path) {
        Ok(s) => s,
        Err(e) => {
            return Some(SortResult {
                path: path.as_path().into(),
                error: Some(e),
            })
        }
    };
    let result = match sort(&file, use_spaces, sort_arrays, line_ending, indents) {
        Ok(json_string) => {
            if !dry_run {
                match write_out(&path, json_string, line_ending) {
                    Ok(_) => None,
                    Err(error) => Some(error),
                }
            } else {
                None
            }
        }
        Err(error) => Some(error),
    };

    Some(SortResult {
        path: path.as_path().into(),
        error: result,
    })
}

fn is_already_sorted(path: &Path, results: &Vec<SortResult>) -> bool {
    results.iter().any(|result| {
        if !result.path.exists() || !path.exists() {
            return false;
        }
        let result_str = result.path.canonicalize().unwrap_or_default();
        let path_str = path.canonicalize().unwrap_or_default();
        result_str == path_str
    })
}

fn is_ignored(path: &Path) -> bool {
    if let Ok(full_path) = path.canonicalize() {
        if let Some(path_str) = full_path.to_str() {
            return IGNORED_FILES.iter().any(|f| path_str.contains(f));
        }
    }
    return false;
}

fn path_to_relative(path: &Path) -> Result<String, Box<dyn Error>> {
    let current = env::current_dir()?.canonicalize()?;

    if let Ok(full) = path.canonicalize() {
        if let Some(full_str) = full.to_str() {
            if let Some(current_str) = current.to_str() {
                return Ok(format!(".{}", full_str.replace(current_str, "")));
            }
        }
    }

    // remove existing './' if exists in current PathBuf
    let re = Regex::new(r"^\./")?;
    let out = match path.to_str() {
        Some(s) => s,
        None => return Err("Path is not valid unicode")?,
    };
    let out = re.replace_all(out, "");

    Ok(format!("./{}", out))
}

fn read_file(path: &Path) -> Result<String, JsonError> {
    if !path.exists() {
        log::debug!("File does not exist");
        return Err(JsonError::NotFound);
    }

    let file = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(error) => {
            log::debug!("Failed to read file: {}", error);
            return Err(JsonError::ReadError);
        }
    };

    Ok(file)
}

fn serialize_json(
    json: &Value,
    whitespace_char: char,
    indents: usize,
    line_ending: &LineEnding,
) -> Result<String, Box<dyn Error>> {
    let mut buf = Vec::new();

    let indent_size = whitespace_char.to_string().repeat(indents);
    let formatter = LineFormatter::new(indent_size.as_bytes(), line_ending.clone());

    let mut ser = Serializer::with_formatter(&mut buf, formatter);
    json.serialize(&mut ser)?;

    Ok(String::from_utf8(buf)?)
}

fn sort_json_value(head: &mut Value, sort_arrays: bool) -> &mut Value {
    if !sort_arrays {
        return head;
    }

    match head {
        Value::Array(list) => {
            if sort_arrays {
                if list.iter().all(|f| f.is_string()) {
                    list.sort_by(|a, b| {
                        a.as_str()
                            .unwrap_or_default()
                            .to_lowercase()
                            .cmp(&b.as_str().unwrap_or_default().to_lowercase())
                    });
                    log::trace!("Sorted array")
                } else {
                    log::trace!("Cannot sort array containing non-strings");
                }
            }
            for item in list.iter_mut() {
                log::trace!("Sorting inner array of array");
                sort_json_value(item, sort_arrays);
            }
        }
        Value::Object(obj) => {
            log::trace!("Sorting object");
            for (key, val) in obj.iter_mut() {
                log::trace!("Sorted object value. key: {}", key);
                sort_json_value(val, sort_arrays);
            }
        }
        _ => {
            log::trace!("type already sorted")
        }
    }

    head
}

fn sort(
    input: &String,
    use_spaces: bool,
    sort_arrays: bool,
    line_ending: &LineEnding,
    indents: usize,
) -> Result<String, JsonError> {
    let mut json: Value = match serde_json::from_str(input) {
        Ok(v) => v,
        Err(error) => {
            log::debug!("Failed to parse json file. error: {}", error);
            return Err(JsonError::ParseError);
        }
    };

    sort_json_value(&mut json, sort_arrays);

    let desired_line_ending: LineEnding = match line_ending {
        // if not specified, use original
        LineEnding::SystemDefault => LineEnding::parse_str(input),
        // else use as configured
        _ => line_ending.clone(),
    };

    let whitespace_char = if use_spaces { ' ' } else { '\t' };
    let json_string = match serialize_json(&json, whitespace_char, indents, &desired_line_ending) {
        Ok(s) => s,
        Err(error) => {
            log::debug!("Serialization error: {}", error);
            return Err(JsonError::WriteError);
        }
    };

    Ok(json_string)
}

fn write_out(
    path: &Path,
    mut json_string: String,
    line_ending: &LineEnding,
) -> Result<(), JsonError> {
    // End file with line ending
    json_string += line_ending.as_str();

    // TODO optimize this by sorting all the file contents in memory first, then saving
    match fs::write(path, json_string) {
        Ok(()) => (),
        Err(error) => {
            log::debug!("File write error: {}", error);
            return Err(JsonError::WriteError);
        }
    };

    Ok(())
}
