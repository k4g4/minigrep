pub mod args;
pub mod error;
pub mod results;

use args::MinigrepArgs;
use error::{MinigrepArgsError, MinigrepError};
use results::MinigrepResults;
use std::{fs, path::Path};

pub fn run(args: MinigrepArgs) -> Result<MinigrepResults, MinigrepError> {
    let mut results = MinigrepResults::new(args.quiet());
    for path in args.paths() {
        if path.is_dir() {
            search_dir(&path, &args, &mut results)?;
        } else if path.is_file() {
            search_file(path, &args, &mut results)?;
        }
    }

    Ok(results)
}

fn search<'a>(
    data_stream: impl Iterator<Item = &'a str>,
    args: &MinigrepArgs,
    results: &mut MinigrepResults,
) -> Result<Vec<String>, MinigrepError> {
    Ok(vec![])
}

fn search_file(
    file_path: &Path,
    args: &MinigrepArgs,
    results: &mut MinigrepResults,
) -> Result<(), MinigrepError> {
    results.add_file(file_path);

    let contents = fs::read_to_string(file_path)?;
    let index = contents
        .find(args.query())
        .ok_or(MinigrepError::NoResults)?;
    let mut line = 0;
    let result: String = contents
        .chars()
        .inspect(|c| {
            if *c == '\n' {
                line += 1;
            }
        })
        .skip(index)
        .take_while(|c| *c != '\n')
        .collect();
    results.add_finding(file_path, &result);

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
            return Err(MinigrepError::BadArgs(MinigrepArgsError::PathInaccessible(
                entry_path,
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod run_tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn search_succeeds() -> Result<(), MinigrepError> {
        let data = ["The quick brown fox", "jumps over", "the lazy dog"];
        let args = MinigrepArgs::build("jumps", &["file.txt"])?;
        let mut results = MinigrepResults::new(false);

        let findings = search(data.into_iter(), &args, &mut results)?;
        assert_eq!(findings, vec!["jumps over"]);

        Ok(())
    }

    #[test]
    fn search_file_succeeds() -> Result<(), MinigrepError> {
        let args = MinigrepArgs::build("dreary", &["test.txt"])?;
        let mut results = MinigrepResults::new(false);
        search_file(args.paths()[0].as_path(), &args, &mut results)?;

        assert_eq!(
            results.findings()[&PathBuf::from("test.txt")][0],
            "How dreary to be somebody!"
        );

        Ok(())
    }

    #[test]
    fn search_dir_succeeds() -> Result<(), MinigrepError> {
        let args = MinigrepArgs::build("dreary", &["test_dir"])?;
        let mut results = MinigrepResults::new(false);
        search_dir(args.paths()[0].as_path(), &args, &mut results)?;

        assert_eq!(
            results.findings()[&PathBuf::from("test_dir/test.txt")][0],
            "How dreary to be somebody!"
        );

        Ok(())
    }
}

pub const HELP: &str =
"Usage: minigrep [OPTION]... PATTERN [FILE]...
Search for PATTERNS in each file or directory provided.
Example: minigrep 'hello world' main.rs tests/

Miscellaneous:
  -h, --help          print this message
  -q, --quiet         only display results
";
