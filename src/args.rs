use crate::error::{MinigrepArgsError, MinigrepError};
use std::path::PathBuf;

pub struct MinigrepArgs {
    query: String,
    paths: Vec<PathBuf>,
    quiet: bool,
}

impl MinigrepArgs {
    pub fn build(query: &str, paths: &[&str]) -> Result<Self, MinigrepError> {
        Self {
            query: String::from(query),
            paths: Vec::<PathBuf>::from_iter(paths.iter().map(PathBuf::from)),
            quiet: false,
        }
        .validate()
    }

    pub fn from_arg_strings(
        args: &mut impl Iterator<Item = String>,
    ) -> Result<Self, MinigrepError> {
        let mut options = vec![];
        let mut query = String::new();
        let mut paths = Vec::<PathBuf>::new();

        while let Some(arg) = args.next() {
            if ["-h", "--help"].contains(&arg.as_str()) {
                return Err(MinigrepError::Help);
            }
            if arg.starts_with('-') {
                options.push(arg);
            } else {
                // all options have been read, just query and path args remaining
                query = arg;
                while let Some(arg) = args.next() {
                    paths.push(PathBuf::from(arg));
                }
                break;
            }
        }

        if query.is_empty() {
            return Err(MinigrepError::BadArgs(MinigrepArgsError::QueryMissing));
        }
        if paths.is_empty() {
            return Err(MinigrepError::BadArgs(MinigrepArgsError::PathMissing));
        }

        let mut quiet = false;
        for option in options {
            if ["-q", "--quiet"].contains(&option.as_str()) {
                quiet = true;
            }
        }

        Self {
            query,
            paths,
            quiet,
        }
        .validate()
    }

    fn validate(self) -> Result<Self, MinigrepError> {
        for path in &self.paths {
            let accessible = path.is_dir() || path.is_file();
            if !path.try_exists().is_ok_and(|success| success) || !accessible {
                return Err(MinigrepError::BadArgs(MinigrepArgsError::PathInaccessible(
                    path.to_path_buf(),
                )));
            }
        }

        Ok(self)
    }

    pub fn query(&self) -> &str {
        self.query.as_str()
    }

    pub fn paths(&self) -> &Vec<PathBuf> {
        &self.paths
    }

    pub fn quiet(&self) -> bool {
        self.quiet
    }
}

#[cfg(test)]
mod minigrep_args_tests {
    use super::*;

    #[test]
    fn minigrep_args_succeeds() -> Result<(), MinigrepError> {
        let query = "query";
        let paths = ["test.txt"];
        let args = MinigrepArgs::build(query, &paths)?;

        assert_eq!(args.query(), query);
        assert_eq!(args.paths()[0].as_os_str(), paths[0]);

        let args = ["query", "test.txt", "test_dir"];
        let args = MinigrepArgs::from_arg_strings(&mut args.iter().map(|&x| x.to_string()))?;

        assert_eq!(args.query(), "query");
        assert_eq!(args.paths()[0].as_os_str(), "test.txt");
        assert_eq!(args.paths()[1].as_os_str(), "test_dir");

        Ok(())
    }

    #[test]
    fn minigrep_args_path_not_found() {
        let query = "query";
        let paths = [""];

        if let Err(error) = MinigrepArgs::build(query, &paths) {
            match error {
                MinigrepError::BadArgs(MinigrepArgsError::PathInaccessible(_)) => {}
                _ => {
                    panic!("Error besides PathInaccessible returned from MinigrepArgs::build.");
                }
            }
        } else {
            panic!("Did not receive PathInaccessible error from MinigrepArgs::build.");
        }
    }
}
