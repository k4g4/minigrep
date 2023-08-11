use std::{
    env, fmt,
    path::{Path, PathBuf},
};

pub enum MinigrepArgsError {
    QueryMissing,
    PathMissing,
    QueryWhitespace,
    PathNotFound(PathBuf),
}

impl fmt::Debug for MinigrepArgsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryMissing => write!(f, "Provide a search query."),
            Self::PathMissing => write!(f, "Provide a file path."),
            Self::QueryWhitespace => write!(f, "Search query cannot include whitespace."),
            Self::PathNotFound(path) => write!(f, "Could not find path '{}'.", path.display()),
        }
    }
}

#[derive(Debug)]
pub struct MinigrepArgs {
    query: String,
    path: PathBuf,
}

impl MinigrepArgs {
    #[cfg(test)]
    pub fn new(query: &str, path: &str) -> Result<Self, MinigrepArgsError> {
        let minigrep_args = Self {
            query: String::from(query),
            path: PathBuf::from(path),
        };
        minigrep_args.validate()?;
        Ok(minigrep_args)
    }

    pub fn from_env_args() -> Result<Self, MinigrepArgsError> {
        let mut args = env::args().skip(1);
        let query = args.next().ok_or(MinigrepArgsError::QueryMissing)?;
        let path = PathBuf::from(args.next().ok_or(MinigrepArgsError::PathMissing)?);
        let minigrep_args = Self { query, path };
        minigrep_args.validate()?;
        Ok(minigrep_args)
    }

    fn validate(&self) -> Result<(), MinigrepArgsError> {
        if self.query.split_whitespace().nth(1).is_some() {
            return Err(MinigrepArgsError::QueryWhitespace);
        }
        if !self.path.try_exists().is_ok_and(|success| success) {
            return Err(MinigrepArgsError::PathNotFound(self.path.to_path_buf()));
        }

        Ok(())
    }

    pub fn query(&self) -> &str {
        self.query.as_str()
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
}