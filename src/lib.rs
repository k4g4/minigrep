pub mod args;
pub mod error;
pub mod results;

use args::MinigrepArgs;
use error::MinigrepError;
use results::MinigrepResults;
use std::{fs::File, io::Read, path::Path, str};

pub fn run(args: MinigrepArgs) -> Result<MinigrepResults, MinigrepError> {
    let mut results = MinigrepResults::new(args.quiet());

    if args.paths().is_empty() {
        // read from stdin
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

fn search<T>(data_stream: T, args: &MinigrepArgs) -> Result<Vec<String>, MinigrepError>
where
    T: Read,
{
    const BUF_SIZE: usize = 4 * 1024;
    const CURR_LINE_SIZE: usize = 256;
    let mut findings = Vec::<String>::new();
    let query_bytes = args.query().as_bytes();
    let mut contents = String::with_capacity(BUF_SIZE);
    let mut take = data_stream.take(BUF_SIZE as u64);
    let mut current_line = Vec::<u8>::with_capacity(CURR_LINE_SIZE);
    let mut query_remainder = query_bytes;
    let mut in_query = false;
    let mut query_found = false;

    loop {
        if take.read_to_string(&mut contents)? == 0 {
            break;
        }
        for &byte in contents.as_bytes() {
            if byte == b'\n' {
                if query_found {
                    findings.push(str::from_utf8(&current_line)?.to_string());
                    query_found = false;
                    in_query = false;
                }
                if in_query {
                    current_line.extend_from_slice(b"\\n");
                } else {
                    current_line.clear();
                }
            } else {
                current_line.push(byte);
            }

            if query_remainder[0] == byte {
                in_query = true;
                query_remainder = &query_remainder[1..];
            } else {
                in_query = false;
                query_remainder = query_bytes;
            }

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
    let file_findings = search(file, args)?;
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

#[cfg(test)]
mod run_tests {
    use super::*;
    use std::{io::Cursor, path::PathBuf};

    #[test]
    fn search_succeeds() -> Result<(), MinigrepError> {
        let data = "The quick brown fox\njumps over\nthe lazy dog";
        let cursor = Cursor::new(data);
        let args = MinigrepArgs::build("\njumps", &["test.txt"], true)?;

        let findings = search(cursor, &args)?;
        assert_eq!(findings, vec!["jumps over"]);

        Ok(())
    }

    #[test]
    fn search_file_succeeds() -> Result<(), MinigrepError> {
        let file_path = "test.txt";
        let path_buf = PathBuf::from(file_path);
        let args = MinigrepArgs::build("dreary", &[file_path], true)?;
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
        let args = MinigrepArgs::build("dreary", &["test_dir"], true)?;
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

pub const HELP: &str = "Usage: minigrep [OPTION]... PATTERN [FILE]...
Search for PATTERNS in each file or directory provided.
Example: minigrep 'hello world' main.rs tests/

Miscellaneous:
  -h, --help          print this message
  -q, --quiet         only output search results
  -r, --recursive     recursively read directories
";
