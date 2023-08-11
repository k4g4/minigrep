use crate::minigrep_error::MinigrepArgsError;
use std::{
    env,
    path::{Path, PathBuf},
};

pub struct MinigrepArgs {
    query: String,
    path: PathBuf,
}

impl MinigrepArgs {
    #[cfg(test)]
    pub fn new(query: &str, path: &str) -> Result<Self, MinigrepArgsError> {
        Self {
            query: String::from(query),
            path: PathBuf::from(path),
        }
        .validate()
    }

    pub fn from_env_args() -> Result<Self, MinigrepArgsError> {
        let mut args = env::args().skip(1);
        let query = args.next().ok_or(MinigrepArgsError::QueryMissing)?;
        let path = PathBuf::from(args.next().ok_or(MinigrepArgsError::PathMissing)?);
        Self { query, path }.validate()
    }

    fn validate(self) -> Result<Self, MinigrepArgsError> {
        if self.query.split_whitespace().nth(1).is_some() {
            return Err(MinigrepArgsError::QueryWhitespace);
        }
        if !self.path.try_exists().is_ok_and(|success| success) {
            return Err(MinigrepArgsError::PathNotFound(self.path.to_path_buf()));
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
