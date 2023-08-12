use crate::error::MinigrepArgsError;
use std::path::{Path, PathBuf};

pub struct MinigrepArgs {
    query: String,
    path: PathBuf,
}

impl MinigrepArgs {
    #[cfg(test)]
    pub fn build(query: &str, path: &str) -> Result<Self, MinigrepArgsError> {
        Self {
            query: String::from(query),
            path: PathBuf::from(path),
        }
        .validate()
    }

    pub fn from_strings(
        args: &mut impl Iterator<Item = String>,
    ) -> Result<Self, MinigrepArgsError> {
        let query = args.next().ok_or(MinigrepArgsError::QueryMissing)?;
        let path = args.next().ok_or(MinigrepArgsError::PathMissing)?;
        let path = PathBuf::from(path);
        Self { query, path }.validate()
    }

    fn validate(self) -> Result<Self, MinigrepArgsError> {
        if self.query.split_whitespace().nth(1).is_some() {
            return Err(MinigrepArgsError::QueryWhitespace);
        }

        let accessible = self.path.is_dir() || self.path.is_file();
        if !self.path.try_exists().is_ok_and(|success| success) || !accessible {
            return Err(MinigrepArgsError::PathInaccessible(self.path.to_path_buf()));
        }

        Ok(self)
    }

    pub fn query(&self) -> &str {
        self.query.as_str()
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
}

#[cfg(test)]
mod minigrep_args_tests {
    use super::*;

    #[test]
    fn minigrep_args_succeeds() -> Result<(), MinigrepArgsError> {
        let query = "query";
        let path = "test.txt";
        let minigrep_args = MinigrepArgs::build(query, path)?;

        assert_eq!(minigrep_args.query(), query);
        assert_eq!(minigrep_args.path().display().to_string(), path);

        let args = vec!["query".to_string(), "test.txt".to_string()];
        let minigrep_args = MinigrepArgs::from_strings(&mut args.into_iter())?;

        assert_eq!(minigrep_args.query(), query);
        assert_eq!(minigrep_args.path().display().to_string(), path);

        Ok(())
    }

    #[test]
    fn minigrep_args_path_not_found() {
        let query = "query";
        let path = "";

        if let Err(error) = MinigrepArgs::build(query, path) {
            match error {
                MinigrepArgsError::PathInaccessible(_) => {}
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

        if let Err(error) = MinigrepArgs::build(query, path) {
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
