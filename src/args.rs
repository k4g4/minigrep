use crate::error::MinigrepError;
use std::path::PathBuf;

pub struct MinigrepArgs {
    query: String,
    paths: Vec<PathBuf>,
    quiet: bool,
    recursive: bool,
}

impl MinigrepArgs {
    pub fn build(query: &str, paths: &[&str], recursive: bool) -> Result<Self, MinigrepError> {
        Self {
            query: String::from(query),
            paths: Vec::<PathBuf>::from_iter(paths.iter().map(PathBuf::from)),
            quiet: false,
            recursive,
        }
        .validate()
    }

    pub fn from_arg_strings<T>(args: T) -> Result<Self, MinigrepError>
    where
        T: IntoIterator<Item = String>,
    {
        let mut options = vec![];
        let mut query = String::new();
        let mut paths = Vec::<PathBuf>::new();
        let mut args_iter = args.into_iter();

        while let Some(arg) = args_iter.next() {
            if ["-h", "--help"].contains(&arg.as_str()) {
                return Err(MinigrepError::Help);
            }
            if arg.starts_with('-') {
                options.push(arg);
            } else {
                // all options have been read, just query and path args remaining
                query = arg;
                paths.extend(args_iter.map(PathBuf::from));
                break;
            }
        }

        if query.is_empty() {
            return Err(MinigrepError::QueryMissing);
        }

        let mut quiet = false;
        let mut recursive = false;
        for option in options {
            if ["-q", "--quiet"].contains(&option.as_str()) {
                quiet = true;
            }
            if ["-r", "--recursive"].contains(&option.as_str()) {
                recursive = true;
            }
            if ["-qr", "-rq"].contains(&option.as_str()) {
                quiet = true;
                recursive = true;
            }
        }

        Self {
            query,
            paths,
            quiet,
            recursive,
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
}

#[cfg(test)]
mod minigrep_args_tests {
    use super::*;

    #[test]
    fn minigrep_args_succeeds() -> Result<(), MinigrepError> {
        let query = "query";
        let paths = ["test.txt"];
        let args = MinigrepArgs::build(query, &paths, true)?;

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

        if let Err(error) = MinigrepArgs::build(query, &paths, true) {
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
