use crate::error::MinigrepError;
use std::{env, path::PathBuf};

pub struct MinigrepArgs {
    query: String,
    paths: Vec<PathBuf>,
    quiet: bool,
    recursive: bool,
    ignore_case: bool,
}

impl MinigrepArgs {
    pub fn build(query: &str, paths: &[&str]) -> Result<Self, MinigrepError> {
        Self {
            query: String::from(query),
            paths: Vec::<PathBuf>::from_iter(paths.iter().map(PathBuf::from)),
            quiet: false,
            recursive: false,
            ignore_case: false,
        }
        .validate()
    }

    pub fn from_env_args<T>(mut args: T) -> Result<Self, MinigrepError>
    where
        T: Iterator<Item = String>,
    {
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
                paths.extend(args.map(PathBuf::from));
                break;
            }
        }

        if query.is_empty() {
            return Err(MinigrepError::QueryMissing);
        }

        let mut quiet = false;
        let mut recursive = false;
        let mut ignore_case = false;
        for option_group in options {
            if option_group.starts_with("--") {
                match option_group.as_str() {
                    "--quiet" => quiet = true,
                    "--recursive" => recursive = true,
                    "--ignore-case" => ignore_case = true,
                    _ => return Err(MinigrepError::UnknownArgument(option_group)),
                }
            } else {
                let option_letters = &option_group[1..];
                for option_letter in option_letters.chars() {
                    match option_letter {
                        'q' => quiet = true,
                        'r' => recursive = true,
                        'i' => ignore_case = true,
                        _ => return Err(MinigrepError::UnknownArgument(option_letter.to_string())),
                    }
                }
            }
        }

        ignore_case |= env::var("IGNORE_CASE").is_ok();

        Self {
            query,
            paths,
            quiet,
            recursive,
            ignore_case,
        }
        .validate()
    }

    fn validate(self) -> Result<Self, MinigrepError> {
        for path in &self.paths {
            let accessible = path.is_dir() || path.is_file();
            if !path.try_exists().is_ok_and(|success| success) || !accessible {
                return Err(MinigrepError::PathInaccessible(path.to_path_buf()));
            }
        }

        Ok(self)
    }

    pub fn set_flags(&mut self, recursive: bool, ignore_case: bool) {
        self.recursive = recursive;
        self.ignore_case = ignore_case;
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

    pub fn recursive(&self) -> bool {
        self.recursive
    }

    pub fn ignore_case(&self) -> bool {
        self.ignore_case
    }
}

#[cfg(test)]
mod minigrep_args_tests {
    use super::*;

    #[test]
    fn minigrep_args_succeeds() -> Result<(), MinigrepError> {
        let query = "query";
        let paths = ["test.txt"];
        let mut args = MinigrepArgs::build(query, &paths)?;
        args.set_flags(true, false);

        assert_eq!(args.query(), query);
        assert_eq!(args.paths()[0].as_os_str(), paths[0]);

        let args = ["query", "test.txt", "test_dir"];
        let args = MinigrepArgs::from_env_args(&mut args.iter().map(|&x| x.to_string()))?;

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
                MinigrepError::PathInaccessible(_) => {}
                _ => {
                    panic!("Error besides PathInaccessible returned from MinigrepArgs::build.");
                }
            }
        } else {
            panic!("Did not receive PathInaccessible error from MinigrepArgs::build.");
        }
    }
}
