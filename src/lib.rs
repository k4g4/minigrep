pub mod args;
pub mod error;
pub mod results;

use args::MinigrepArgs;
use error::MinigrepError;
use results::MinigrepResults;
use std::{
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
    str,
};

pub fn run(args: MinigrepArgs) -> Result<MinigrepResults, MinigrepError> {
    let mut results = MinigrepResults::new(args.quiet());

    if args.paths().is_empty() {
        search_stdin(&args, &mut results)?;
    }

    for path in args.paths() {
        if path.is_dir() {
            if !args.recursive() {
                results.add_ignored_dir(path);
                continue;
            }
            search_dir(path, &args, &mut results)?;
        } else if path.is_file() {
            search_file(path, &args, &mut results)?;
        }
    }

    Ok(results)
}

pub fn search<T>(reader: T, query: &str, ignore_case: bool) -> Result<Vec<String>, MinigrepError>
where
    T: Read,
{
    const BUF_SIZE: usize = 1;
    const CURR_LINE_SIZE: usize = 256;
    let mut findings = Vec::<String>::new();
    let mut contents = String::with_capacity(BUF_SIZE);
    let mut take = reader.take(BUF_SIZE as u64);
    let mut current_line = Vec::<u8>::with_capacity(CURR_LINE_SIZE);
    let query_bytes = query.as_bytes();
    let mut query_remainder = query_bytes;
    let mut inside_query = false;
    let mut query_found = false;

    while take.read_to_string(&mut contents)? > 0 {
        for &byte in contents.as_bytes() {
            match byte {
                b'\n' => {
                    if query_found {
                        findings.push(str::from_utf8(&current_line)?.to_string());
                        query_found = false;
                        inside_query = false;
                    }
                    if inside_query {
                        current_line.extend_from_slice(b"\\n");
                    } else {
                        current_line.clear();
                    }
                }
                b'\r' => {
                    current_line.extend_from_slice(b"\\r");
                }
                _ => {
                    current_line.push(byte);
                }
            }

            inside_query = if ignore_case && byte.is_ascii() {
                query_remainder[0].eq_ignore_ascii_case(&byte)
            } else {
                query_remainder[0] == byte
            };

            query_remainder = if inside_query {
                &query_remainder[1..]
            } else {
                query_bytes
            };

            if query_remainder.is_empty() {
                query_found = true;
                query_remainder = query_bytes;
            }
        }
        contents.clear();
        take.set_limit(BUF_SIZE as u64);
    }
    if query_found {
        findings.push(str::from_utf8(&current_line)?.to_string());
    }

    Ok(findings)
}

fn search_file(
    file_path: &Path,
    args: &MinigrepArgs,
    results: &mut MinigrepResults,
) -> Result<(), MinigrepError> {
    let file = File::open(file_path)?;
    let file_findings = search(file, args.query(), args.ignore_case())?;
    results.add_findings(file_path, file_findings);

    Ok(())
}

fn search_dir(
    dir_path: &Path,
    args: &MinigrepArgs,
    results: &mut MinigrepResults,
) -> Result<(), MinigrepError> {
    for entry in dir_path.read_dir()? {
        let entry_path = entry?.path();
        if entry_path.is_dir() {
            search_dir(&entry_path, args, results)?;
        } else if entry_path.is_file() {
            search_file(&entry_path, args, results)?;
        } else {
            return Err(MinigrepError::PathInaccessible(entry_path));
        }
    }

    Ok(())
}

fn search_stdin(args: &MinigrepArgs, results: &mut MinigrepResults) -> Result<(), MinigrepError> {
    let stdin = io::stdin().lock();
    let stdin_findings = search(stdin, args.query(), args.ignore_case())?;
    results.add_findings(&PathBuf::from("stdin"), stdin_findings);

    Ok(())
}

#[cfg(test)]
mod run_tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn search_succeeds() -> Result<(), MinigrepError> {
        let data = "The quick brown fox\njumps over\nthe lazy dog";
        let cursor = Cursor::new(data);
        let findings = search(cursor, "\njumps", false)?;
        assert_eq!(findings, vec!["jumps over"]);

        Ok(())
    }

    #[test]
    fn search_ignore_case_succeeds() -> Result<(), MinigrepError> {
        let data = "The quick brown fox\njumps over\nthe lazy dog";
        let cursor = Cursor::new(data);
        let findings = search(cursor, "BROWN FOX", true)?;
        assert_eq!(findings, vec!["The quick brown fox"]);

        let data = "The quick brown fox\njumps over\nthe Lazy Dog";
        let cursor = Cursor::new(data);
        let findings = search(cursor, "lazy dog", true)?;
        assert_eq!(findings, vec!["the Lazy Dog"]);

        Ok(())
    }

    #[test]
    fn search_file_succeeds() -> Result<(), MinigrepError> {
        let file_path = "test.txt";
        let path_buf = PathBuf::from(file_path);
        let mut args = MinigrepArgs::build("dreary", &[file_path])?;
        args.set_flags(true, false);
        let mut results = MinigrepResults::new(false);
        search_file(args.paths()[0].as_path(), &args, &mut results)?;

        assert_eq!(results.findings()[&path_buf].len(), 1);
        assert_eq!(
            results.findings()[&path_buf][0],
            "How dreary to be somebody!"
        );

        Ok(())
    }

    #[test]
    fn search_dir_succeeds() -> Result<(), MinigrepError> {
        let mut args = MinigrepArgs::build("dreary", &["test_dir"])?;
        args.set_flags(true, false);
        let mut results = MinigrepResults::new(false);
        search_dir(args.paths()[0].as_path(), &args, &mut results)?;

        assert_eq!(results.findings().len(), 3);
        assert_eq!(
            results.findings()[&PathBuf::from("test_dir/test.txt")].len(),
            1
        );
        assert_eq!(
            results.findings()[&PathBuf::from("test_dir/inner_dir/test.txt")].len(),
            4
        );
        assert_eq!(
            results.findings()[&PathBuf::from("test_dir/test2.txt")].len(),
            0
        );

        Ok(())
    }
}

pub const HELP: &str = "\
Usage: minigrep [OPTION]... PATTERN [FILE]...
Search for PATTERNS in each file or directory provided.
Example: minigrep 'hello world' main.rs tests/

Miscellaneous:
  -h, --help          print this message
  -q, --quiet         only output search results
  -r, --recursive     recursively read directories
";
