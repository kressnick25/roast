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

const INVALID_PATH: &str = "INVALID_PATH";
const IGNORED_FILES: &[&str] = &[
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

impl Display for SortResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path_str = path_to_relative(&self.path).unwrap_or(INVALID_PATH.into());

        if self.success() {
            write!(f, "{} - {}", path_str, "OK".green().bold())
        } else {
            let err_msg = format!("{:?}", self.error.as_ref().expect("Not possible"))
                .red()
                .bold();
            write!(f, "{path_str} - {err_msg}")
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
    files: &[PathBuf],
    line_ending: &LineEnding,
    use_spaces: bool,
    sort_arrays: bool,
    indents: usize,
    dry_run: bool,
) -> Vec<SortResult> {
    let mut results: Vec<SortResult> = vec![];

    let all_paths = collect_sortables(files);

    for path in all_paths {
        let res = sort_path(
            &path,
            dry_run,
            line_ending,
            use_spaces,
            sort_arrays,
            indents,
        );
        if let Some(r) = res {
            results.push(r)
        }
    }

    results
}

fn sort_path(
    path: &Path,
    dry_run: bool,
    line_ending: &LineEnding,
    use_spaces: bool,
    sort_arrays: bool,
    indents: usize,
) -> Option<SortResult> {
    if !path.exists() {
        return Some(SortResult {
            path: path.into(),
            error: Some(JsonError::NotFound),
        });
    }

    let file: String = match read_file(path) {
        Ok(s) => s,
        Err(e) => {
            return Some(SortResult {
                path: path.into(),
                error: Some(e),
            })
        }
    };
    let result = match sort_json_string(&file, use_spaces, sort_arrays, line_ending, indents) {
        Ok(json_string) => {
            if !dry_run {
                write_out(path, json_string).err()
            } else {
                None
            }
        }
        Err(error) => Some(error),
    };

    Some(SortResult {
        path: path.into(),
        error: result,
    })
}

fn is_ignored(path: &Path) -> bool {
    if let Ok(full_path) = path.canonicalize() {
        if let Some(path_str) = full_path.to_str() {
            return IGNORED_FILES.iter().any(|f| path_str.contains(f));
        }
    }

    false
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
        _ => return Err("Path is not valid unicode")?,
    };
    let out = re.replace_all(out, "");

    Ok(format!("./{out}"))
}

fn read_file(path: &Path) -> Result<String, JsonError> {
    if !path.exists() {
        log::debug!("File does not exist");
        return Err(JsonError::NotFound);
    }

    let file = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(error) => {
            log::debug!("Failed to read file: {error}");
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
                log::trace!("Sorted object value. key: {key}");
                sort_json_value(val, sort_arrays);
            }
        }
        _ => {
            log::trace!("type already sorted")
        }
    }

    head
}

pub fn sort_json_string(
    input: &str,
    use_spaces: bool,
    sort_arrays: bool,
    line_ending: &LineEnding,
    indents: usize,
) -> Result<String, JsonError> {
    let mut json: Value = match serde_json::from_str(input) {
        Ok(v) => v,
        Err(error) => {
            log::debug!("Failed to parse json file. error: {error}");
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
    let mut json_string =
        match serialize_json(&json, whitespace_char, indents, &desired_line_ending) {
            Ok(s) => s,
            Err(error) => {
                log::debug!("Serialization error: {error}");
                return Err(JsonError::WriteError);
            }
        };

    // End file with line ending
    json_string += desired_line_ending.as_str();

    Ok(json_string)
}

fn write_out(path: &Path, json_string: String) -> Result<(), JsonError> {
    // TODO optimize this by sorting all the file contents in memory first, then saving
    match fs::write(path, json_string) {
        Ok(()) => (),
        Err(error) => {
            log::debug!("File write error: {error}");
            return Err(JsonError::WriteError);
        }
    };

    Ok(())
}

fn collect_sortables(roots: &[PathBuf]) -> Vec<PathBuf> {
    let mut results: Vec<PathBuf> = vec![];

    for root in roots {
        if root.is_dir() {
            for entry in WalkDir::new(root)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file())
            {
                let entry_path = entry.path();
                if is_ignored(entry_path) || path_in_vec(entry_path, &results) {
                    log::debug!("Ignored: {:?}", entry_path.to_str());
                    continue;
                }
                results.push(entry_path.to_path_buf());
            }
        } else {
            if is_ignored(root) || path_in_vec(root, &results) {
                log::debug!("Ignored: {:?}", root.to_str());
                continue;
            }
            results.push(root.to_path_buf())
        }
    }

    results
}

fn path_in_vec(path: &Path, list: &[PathBuf]) -> bool {
    list.iter().any(|result| {
        if !result.exists() || !path.exists() {
            return false;
        }
        let result_str = result.canonicalize().unwrap_or_default();
        let path_str = path.canonicalize().unwrap_or_default();
        result_str == path_str
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(windows)]
    const EOL: &str = "\r\n";
    #[cfg(not(windows))]
    const EOL: &str = "\n";

    #[test]
    fn sort_arrays() -> Result<(), String> {
        let input: String = r#"["a", "A", "z", "Z", "m", "M"]"#.into();
        let result = sort_json_string(&input, true, true, &LineEnding::Lf, 2).unwrap();

        let expected: String = r#"[
  "a",
  "A",
  "m",
  "M",
  "z",
  "Z"
]
"#
        .into();

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn no_sort_arrays() -> Result<(), String> {
        let input: String = r#"["a", "A", "z", "Z", "m", "M"]"#.into();
        let result = sort_json_string(&input, true, false, &LineEnding::Lf, 2).unwrap();

        let expected: String = r#"[
  "a",
  "A",
  "z",
  "Z",
  "m",
  "M"
]
"#
        .into();

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn sort_arrays_deep_objects() -> Result<(), String> {
        let input: String = r#"{
          "a": {
            "b": [
              {
                "c": "d"
              },
              ["z", "m", "A"]
            ]
          }
        }"#
        .into();

        let result = sort_json_string(&input, true, true, &LineEnding::Lf, 2).unwrap();

        let expected: String = r#"{
  "a": {
    "b": [
      {
        "c": "d"
      },
      [
        "A",
        "m",
        "z"
      ]
    ]
  }
}
"#
        .into();

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn no_sort_arrays_deep_objects() -> Result<(), String> {
        let input: String = r#"{
          "a": {
            "b": [
              {
                "c": "d"
              },
              ["z", "m", "A"]
            ]
          }
        }"#
        .into();

        let result = sort_json_string(&input, true, false, &LineEnding::Lf, 2).unwrap();

        let expected: String = r#"{
  "a": {
    "b": [
      {
        "c": "d"
      },
      [
        "z",
        "m",
        "A"
      ]
    ]
  }
}
"#
        .into();

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn indentation_3_spaces() -> Result<(), String> {
        let input: String = r#"{
  "z": 1,
  "a": 2
}
"#
        .into();

        let result = sort_json_string(&input, true, false, &LineEnding::Lf, 3).unwrap();

        let expected: String = r#"{
   "a": 2,
   "z": 1
}
"#
        .into();

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn indentation_3_tabs() -> Result<(), String> {
        let input: String = r#"{
  "z": 1,
  "a": 2
}
"#
        .into();

        let result = sort_json_string(&input, false, false, &LineEnding::Lf, 3).unwrap();

        let expected: String = "{
\t\t\t\"a\": 2,
\t\t\t\"z\": 1
}
"
        .into();

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn indentation_3_tabs_array() -> Result<(), String> {
        let input: String = "[\n  \"z\",\n  \"a\"\n]".into();

        let result = sort_json_string(&input, false, true, &LineEnding::Lf, 3).unwrap();

        let expected: String = "[
\t\t\t\"a\",
\t\t\t\"z\"
]
"
        .into();

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn line_endings_system() -> Result<(), String> {
        let input: String = format!(r#"[{EOL}  {{{EOL}    "a": "y",{EOL}    "b": "b"{EOL}  }},{EOL}  {{{EOL}    "c": "r",{EOL}    "p": "d"{EOL}  }}{EOL}]{EOL}"#);

        let result = sort_json_string(&input, true, false, &LineEnding::SystemDefault, 2).unwrap();

        assert_eq!(result, input);
        Ok(())
    }

    #[test]
    fn line_endings_preseve_original_crlf() -> Result<(), String> {
        let input: String = format!(r#"[{0}  {{{0}    "a": "y",{0}    "b": "b"{0}  }},{0}  {{{0}    "c": "r",{0}    "p": "d"{0}  }}{0}]{0}"#, "\r\n");

        let result = sort_json_string(&input, true, false, &LineEnding::SystemDefault, 2).unwrap();

        assert_eq!(result, input);
        Ok(())
    }

    #[test]
    fn line_endings_preseve_original_lf() -> Result<(), String> {
        let input: String = format!(r#"[{0}  {{{0}    "a": "y",{0}    "b": "b"{0}  }},{0}  {{{0}    "c": "r",{0}    "p": "d"{0}  }}{0}]{0}"#, "\n");

        let result = sort_json_string(&input, true, false, &LineEnding::SystemDefault, 2).unwrap();

        assert_eq!(result, input);
        Ok(())
    }

    #[test]
    fn line_endings_crlf_in_cr_out() -> Result<(), String> {
        let input: String = format!(r#"[{0}  {{{0}    "a": "y",{0}    "b": "b"{0}  }},{0}  {{{0}    "c": "r",{0}    "p": "d"{0}  }}{0}]{0}"#, "\r\n");

        let result = sort_json_string(&input, true, false, &LineEnding::Cr, 2).unwrap();

        assert_eq!(result, input.replace("\r\n", "\r"));
        Ok(())
    }

    #[test]
    fn line_endings_crlf_in_lf_out() -> Result<(), String> {
        let input: String = format!(r#"[{0}  {{{0}    "a": "y",{0}    "b": "b"{0}  }},{0}  {{{0}    "c": "r",{0}    "p": "d"{0}  }}{0}]{0}"#, "\r\n");

        let result = sort_json_string(&input, true, false, &LineEnding::Lf, 2).unwrap();

        assert_eq!(result, input.replace("\r\n", "\n"));
        Ok(())
    }

    #[test]
    fn line_endings_lf_in_crlf_out() -> Result<(), String> {
        let input: String = format!(r#"[{0}  {{{0}    "a": "y",{0}    "b": "b"{0}  }},{0}  {{{0}    "c": "r",{0}    "p": "d"{0}  }}{0}]{0}"#, "\n");

        let result = sort_json_string(&input, true, false, &LineEnding::CrLf, 2).unwrap();

        assert_eq!(result, input.replace("\n", "\r\n"));
        Ok(())
    }

    #[test]
    fn large_complex() -> Result<(), String> {
        let minified: String = r#"{"root":true,"env":{"es6":true,"node":true},"extends":["eslint:recommended","plugin:prettier/recommended"],"parserOptions":{"ecmaVersion":2018,"sourceType":"module"},"plugins":["ava","scanjs-rules","no-unsanitized","import"],"rules":{"ava/assertion-arguments":"error","ava/max-asserts":["off",5],"ava/no-async-fn-without-await":"error","ava/no-cb-test":"off","ava/no-duplicate-modifiers":"error","ava/no-identical-title":"error","ava/no-invalid-end":"error","ava/no-nested-tests":"error","ava/no-only-test":"error","ava/no-skip-assert":"error","ava/no-skip-test":"error","ava/no-statement-after-end":"error","ava/no-todo-implementation":"error","ava/no-todo-test":"warn","ava/no-unknown-modifiers":"error","ava/prefer-async-await":"error","ava/prefer-power-assert":"off","ava/test-ended":"error","ava/test-title":["error","if-multiple"],"ava/use-t":"error","ava/use-t-well":"error","ava/use-test":"error","ava/use-true-false":"error","curly":"error","import/no-extraneous-dependencies":["error",{"devDependencies":["**/*test.js","test/**/*.*","rollup.config.js"]}],"no-constant-condition":["error",{"checkLoops":false}],"no-console":"off","no-else-return":"error","no-inner-declarations":"error","no-unneeded-ternary":"error","no-useless-return":"error","no-var":"error","one-var":["error","never"],"prefer-arrow-callback":"error","prefer-const":"error","prefer-template":"error","strict":"error","scanjs-rules/accidental_assignment":1,"scanjs-rules/assign_to_hostname":1,"scanjs-rules/assign_to_href":1,"scanjs-rules/assign_to_location":1,"scanjs-rules/assign_to_onmessage":1,"scanjs-rules/assign_to_pathname":1,"scanjs-rules/assign_to_protocol":1,"scanjs-rules/assign_to_search":1,"scanjs-rules/assign_to_src":1,"scanjs-rules/call_Function":1,"scanjs-rules/call_addEventListener":1,"scanjs-rules/call_addEventListener_deviceproximity":1,"scanjs-rules/call_addEventListener_message":1,"scanjs-rules/call_connect":1,"scanjs-rules/call_eval":1,"scanjs-rules/call_execScript":1,"scanjs-rules/call_hide":1,"scanjs-rules/call_open_remote=true":1,"scanjs-rules/call_parseFromString":1,"scanjs-rules/call_setImmediate":1,"scanjs-rules/call_setInterval":1,"scanjs-rules/call_setTimeout":1,"scanjs-rules/identifier_indexedDB":1,"scanjs-rules/identifier_localStorage":1,"scanjs-rules/identifier_sessionStorage":1,"scanjs-rules/new_Function":1,"scanjs-rules/property_addIdleObserver":1,"scanjs-rules/property_createContextualFragment":1,"scanjs-rules/property_crypto":1,"scanjs-rules/property_geolocation":1,"scanjs-rules/property_getUserMedia":1,"scanjs-rules/property_indexedDB":1,"scanjs-rules/property_localStorage":1,"scanjs-rules/property_mgmt":1,"scanjs-rules/property_sessionStorage":1,"symbol-description":"error","yoda":["error","never",{"exceptRange":true}]}}"#.into();

        let prettified = "{
\t\"env\": {
\t\t\"es6\": true,
\t\t\"node\": true
\t},
\t\"extends\": [
\t\t\"eslint:recommended\",
\t\t\"plugin:prettier/recommended\"
\t],
\t\"parserOptions\": {
\t\t\"ecmaVersion\": 2018,
\t\t\"sourceType\": \"module\"
\t},
\t\"plugins\": [
\t\t\"ava\",
\t\t\"scanjs-rules\",
\t\t\"no-unsanitized\",
\t\t\"import\"
\t],
\t\"root\": true,
\t\"rules\": {
\t\t\"ava/assertion-arguments\": \"error\",
\t\t\"ava/max-asserts\": [
\t\t\t\"off\",
\t\t\t5
\t\t],
\t\t\"ava/no-async-fn-without-await\": \"error\",
\t\t\"ava/no-cb-test\": \"off\",
\t\t\"ava/no-duplicate-modifiers\": \"error\",
\t\t\"ava/no-identical-title\": \"error\",
\t\t\"ava/no-invalid-end\": \"error\",
\t\t\"ava/no-nested-tests\": \"error\",
\t\t\"ava/no-only-test\": \"error\",
\t\t\"ava/no-skip-assert\": \"error\",
\t\t\"ava/no-skip-test\": \"error\",
\t\t\"ava/no-statement-after-end\": \"error\",
\t\t\"ava/no-todo-implementation\": \"error\",
\t\t\"ava/no-todo-test\": \"warn\",
\t\t\"ava/no-unknown-modifiers\": \"error\",
\t\t\"ava/prefer-async-await\": \"error\",
\t\t\"ava/prefer-power-assert\": \"off\",
\t\t\"ava/test-ended\": \"error\",
\t\t\"ava/test-title\": [
\t\t\t\"error\",
\t\t\t\"if-multiple\"
\t\t],
\t\t\"ava/use-t\": \"error\",
\t\t\"ava/use-t-well\": \"error\",
\t\t\"ava/use-test\": \"error\",
\t\t\"ava/use-true-false\": \"error\",
\t\t\"curly\": \"error\",
\t\t\"import/no-extraneous-dependencies\": [
\t\t\t\"error\",
\t\t\t{
\t\t\t\t\"devDependencies\": [
\t\t\t\t\t\"**/*test.js\",
\t\t\t\t\t\"test/**/*.*\",
\t\t\t\t\t\"rollup.config.js\"
\t\t\t\t]
\t\t\t}
\t\t],
\t\t\"no-console\": \"off\",
\t\t\"no-constant-condition\": [
\t\t\t\"error\",
\t\t\t{
\t\t\t\t\"checkLoops\": false
\t\t\t}
\t\t],
\t\t\"no-else-return\": \"error\",
\t\t\"no-inner-declarations\": \"error\",
\t\t\"no-unneeded-ternary\": \"error\",
\t\t\"no-useless-return\": \"error\",
\t\t\"no-var\": \"error\",
\t\t\"one-var\": [
\t\t\t\"error\",
\t\t\t\"never\"
\t\t],
\t\t\"prefer-arrow-callback\": \"error\",
\t\t\"prefer-const\": \"error\",
\t\t\"prefer-template\": \"error\",
\t\t\"scanjs-rules/accidental_assignment\": 1,
\t\t\"scanjs-rules/assign_to_hostname\": 1,
\t\t\"scanjs-rules/assign_to_href\": 1,
\t\t\"scanjs-rules/assign_to_location\": 1,
\t\t\"scanjs-rules/assign_to_onmessage\": 1,
\t\t\"scanjs-rules/assign_to_pathname\": 1,
\t\t\"scanjs-rules/assign_to_protocol\": 1,
\t\t\"scanjs-rules/assign_to_search\": 1,
\t\t\"scanjs-rules/assign_to_src\": 1,
\t\t\"scanjs-rules/call_Function\": 1,
\t\t\"scanjs-rules/call_addEventListener\": 1,
\t\t\"scanjs-rules/call_addEventListener_deviceproximity\": 1,
\t\t\"scanjs-rules/call_addEventListener_message\": 1,
\t\t\"scanjs-rules/call_connect\": 1,
\t\t\"scanjs-rules/call_eval\": 1,
\t\t\"scanjs-rules/call_execScript\": 1,
\t\t\"scanjs-rules/call_hide\": 1,
\t\t\"scanjs-rules/call_open_remote=true\": 1,
\t\t\"scanjs-rules/call_parseFromString\": 1,
\t\t\"scanjs-rules/call_setImmediate\": 1,
\t\t\"scanjs-rules/call_setInterval\": 1,
\t\t\"scanjs-rules/call_setTimeout\": 1,
\t\t\"scanjs-rules/identifier_indexedDB\": 1,
\t\t\"scanjs-rules/identifier_localStorage\": 1,
\t\t\"scanjs-rules/identifier_sessionStorage\": 1,
\t\t\"scanjs-rules/new_Function\": 1,
\t\t\"scanjs-rules/property_addIdleObserver\": 1,
\t\t\"scanjs-rules/property_createContextualFragment\": 1,
\t\t\"scanjs-rules/property_crypto\": 1,
\t\t\"scanjs-rules/property_geolocation\": 1,
\t\t\"scanjs-rules/property_getUserMedia\": 1,
\t\t\"scanjs-rules/property_indexedDB\": 1,
\t\t\"scanjs-rules/property_localStorage\": 1,
\t\t\"scanjs-rules/property_mgmt\": 1,
\t\t\"scanjs-rules/property_sessionStorage\": 1,
\t\t\"strict\": \"error\",
\t\t\"symbol-description\": \"error\",
\t\t\"yoda\": [
\t\t\t\"error\",
\t\t\t\"never\",
\t\t\t{
\t\t\t\t\"exceptRange\": true
\t\t\t}
\t\t]
\t}
}\n";

        let result = sort_json_string(&minified, false, false, &LineEnding::Lf, 1).unwrap();

        assert_eq!(result, prettified);
        Ok(())
    }
}
