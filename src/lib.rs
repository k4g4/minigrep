pub mod args;
pub mod error;
pub mod results;

use args::MinigrepArgs;
use error::{MinigrepArgsError, MinigrepError};
use results::MinigrepResults;
use std::{fs, path::Path};

pub fn run(args: MinigrepArgs) -> Result<MinigrepResults, MinigrepError> {
    let mut results = MinigrepResults::new();
    if args.path().is_dir() {
        search_dir(args.path(), &args, &mut results)?;
    } else if args.path().is_file() {
        search_file(args.path(), &args, &mut results)?;
    }

    Ok(results)
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

    #[test]
    fn search_file_succeeds() -> Result<(), MinigrepError> {
        let args = MinigrepArgs::build("query", "test.txt")?;
        let mut results = MinigrepResults::new();
        search_file(args.path(), &args, &mut results)?;

        assert!(true);

        Ok(())
    }

    #[test]
    fn search_dir_succeeds() -> Result<(), MinigrepError> {
        let args = MinigrepArgs::build("query", "test_dir")?;
        let mut results = MinigrepResults::new();
        search_dir(args.path(), &args, &mut results)?;

        assert!(true);

        Ok(())
    }
}
