mod minigrep_args;

use crate::minigrep_args::{MinigrepArgs, MinigrepArgsError};

fn main() -> Result<(), MinigrepArgsError> {
    let minigrep_args = MinigrepArgs::from_env_args()?;
    println!(
        "Running minigrep with query '{}' on file at path '{}'.",
        minigrep_args.query(),
        minigrep_args.path().display()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minigrep_args_succeeds() -> Result<(), MinigrepArgsError> {
        let query = "query";
        let path = "test.txt";
        let minigrep_args = MinigrepArgs::new(query, path)?;

        assert_eq!(minigrep_args.query(), query);
        assert_eq!(minigrep_args.path().display().to_string(), path);

        Ok(())
    }

    #[test]
    fn minigrep_args_path_not_found() {
        let query = "query";
        let path = "";

        if let Err(error) = MinigrepArgs::new(query, path) {
            match error {
                MinigrepArgsError::PathNotFound(_) => {}
                _ => {
                    panic!("Error besides PathNotFound returned from MinigrepArgs::new.");
                }
            }
        } else {
            panic!("Did not receive PathNotFound error from MinigrepArgs::new.");
        }
    }

    #[test]
    fn minigrep_args_whitespace_query() {
        let query = "query query";
        let path = "test.txt";

        if let Err(error) = MinigrepArgs::new(query, path) {
            match error {
                MinigrepArgsError::QueryWhitespace => {}
                _ => {
                    panic!("Error besides QueryWhitespace returned from MinigrepArgs::new.");
                }
            }
        } else {
            panic!("Did not receive QueryWhitespace error from MinigrepArgs::new.");
        }
    }
}
