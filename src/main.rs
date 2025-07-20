use clap::Parser;
use colored::*;
use git2::{Repository, Status};
use log::{Level, LevelFilter, Metadata, Record};
use sort::sort_json_string;
use std::fmt::Display;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::exit;
use std::{env, io};

mod formatter;
mod lines;
mod sort;

use crate::lines::LineEnding;
use crate::sort::{sort_files, SortResult};

const APP_NAME: &str = "roast";
const APP_VERSION: &str = "0.3.0";
const APP_AUTHOR: &str = "Nicholas Kress";
const APP_ABOUT: &str = "Sort JSON files by key.
Rust implementation of the npm package: json-sort-cli.";

const INDENT_SIZE_SPACE: usize = 2;
const INDENT_SIZE_TAB: usize = 1;

static LOGGER: SimpleLogger = SimpleLogger;

#[derive(Debug, Parser)]
#[command(name = APP_NAME, version = APP_VERSION, author = APP_AUTHOR, about = APP_ABOUT)]
struct Args {
    /// Also sort any arrays if they contain only string elements
    #[clap(long, short = 'a')]
    arrays: bool,

    /// Only list all the files to be processed
    #[clap(long, short = 'd')]
    dry: bool,

    /// Sort any JSON files tracked by git, that have a modified status. Will not modify any untracked, staged, or ignored files
    #[clap(long, short = 'g')]
    git: bool,

    /// How many spaces/tabs to use (default: 2 -> spaces, 1 -> tabs)
    #[clap(long = "indentationCount", short = 'i', default_value = "0")]
    indents: usize,

    /// Set to "cr", "crlf" or "lf". Otherwise, the original line ending of the file is used
    #[clap(long = "lineEnding", short = 'l', default_value = LineEnding::SystemDefault.as_str())]
    #[arg(value_parser = LineEnding::from_str)]
    line_ending: LineEnding,

    /// Suppress output
    #[clap(long)]
    silent: bool,

    /// Use spaces for JSON file indentation (default uses tabs)
    #[clap(long, short = 's')]
    spaces: bool,

    /// Enable verbose output for debugging
    #[clap(long, short = 'v')]
    verbose: bool,

    /// Space separated list of file paths to sort. 
    /// If no paths are supplied, roast will read JSON from stdin and write to stdout.
    files: Vec<PathBuf>,
}

impl Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Args {{
    sort arrays: {:?}
    dry run: {:?}
    indents: {:?}
    line ending: {:?}
    use spaces: {:?}
    verbose output: {:?}
}}",
            self.arrays, self.dry, self.indents, self.line_ending, self.spaces, self.verbose
        )
    }
}

pub struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            match record.level() {
                Level::Debug | Level::Trace => {
                    eprint!(
                        "{} - {}:{} - ",
                        record.level(),
                        record.file().unwrap_or("unknown_file"),
                        record.line().unwrap_or(0)
                    );
                }
                Level::Info => eprintln!("{}", record.args()),
                Level::Warn => eprintln!("{}", format!("{}", record.args()).yellow()),
                Level::Error => eprintln!("{}", format!("{}", record.args()).red()),
            }
        }
    }

    fn flush(&self) {}
}

fn get_git_modified() -> Result<Vec<PathBuf>, git2::Error> {
    let dir = env::current_dir().unwrap_or(PathBuf::from("."));
    let repo = Repository::open(dir)?;
    let statuses = repo.statuses(None)?;

    let res = statuses
        .iter()
        .filter(|se| {
            let s: Status = se.status();
            // index = staged, wt + not new = tracked, unstaged
            s.is_wt_modified() || s.is_wt_renamed() || s.is_wt_typechange()
        })
        .filter_map(|s| s.path().map(PathBuf::from))
        .collect();

    Ok(res)
}

fn sort_result_output(results: Vec<SortResult>) -> String {
    let ok_count = results.iter().filter(|r| r.success()).count();
    let fail_count = results.len() - ok_count;

    let out: String;
    if fail_count > 0 {
        let ok = format!("{ok_count} files sorted").green();
        let fail = format!("{fail_count} files could not be sorted").red();
        out = format!("{ok}\n{fail}").bold().to_string();
    } else if ok_count == 0 {
        out = "The inputs don't lead to any json files! Exiting."
            .red()
            .to_string();
    } else {
        out = format!("{ok_count} files sorted")
            .green()
            .bold()
            .to_string();
    }

    out
}

fn main() {
    // CLI args
    let args = Args::parse();

    cli(args)
}

fn cli(args: Args) {
    // Configure logging
    let _ = log::set_logger(&LOGGER);
    log::set_max_level(LevelFilter::Info);
    // --verbose overrides --silent
    if args.verbose {
        log::set_max_level(LevelFilter::Debug);
    } else if args.silent {
        log::set_max_level(LevelFilter::Error);
    }

    log::debug!("{args}");

    let indents: usize = if args.indents == 0 {
        if args.spaces {
            INDENT_SIZE_SPACE
        } else {
            INDENT_SIZE_TAB
        }
    } else {
        args.indents
    };

    let files: Vec<PathBuf>;
    if args.git {
        log::debug!("Reading paths from git");
        files = match get_git_modified() {
            Ok(f) => f,
            Err(err) => {
                log::debug!("Error reading git repo status: {err}");
                log::error!("fatal: not a git repository");
                std::process::exit(1);
            }
        }
    } else if !args.files.is_empty() {
        log::debug!("Reading paths from argv");
        files = args.files;
    } else {
        log::debug!("Reading from stdin");
        io_mode(&args, indents);
        std::process::exit(0)
    }

    let results = sort_files(
        &files,
        &args.line_ending,
        args.spaces,
        args.arrays,
        indents,
        args.dry,
    );

    for result in results.iter() {
        log::info!("{result}")
    }

    log::info!("");
    if args.dry {
        log::info!(
            "{}\n{}",
            "--- DRY RUN ---".yellow().bold(),
            sort_result_output(results)
        );
    } else {
        log::info!("{}", sort_result_output(results))
    }
}

fn io_mode(args: &Args, indents: usize) {
    let stdin = io::stdin();
    let mut input = Vec::new();
    {
        let mut handle = stdin.lock();
        let res = handle.read_to_end(&mut input);
        if res.is_err() || res.is_ok_and(|x| x == 0) {
            log::info!("No input");
            exit(1);
        }
    }
    let s_input = match String::from_utf8(input) {
        Ok(s) => s,
        Err(e) => {
            log::error!("Error parsing input : {e}");
            exit(1);
        }
    };

    match sort_json_string(
        &s_input,
        args.spaces,
        args.arrays,
        &args.line_ending,
        indents,
    ) {
        Ok(s) => {
            print!("{s}");
            io::stdout().flush().unwrap();
            exit(0);
        }
        Err(e) => {
            log::error!("Error {e:?}");
            exit(1);
        }
    }
}
