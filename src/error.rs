use std::{fmt, io, path::PathBuf};

pub enum MinigrepError {
    BadArgs(MinigrepArgsError),
    IoError(io::Error),
    NoResults,
}

impl From<io::Error> for MinigrepError {
    fn from(error: io::Error) -> Self {
        MinigrepError::IoError(error)
    }
}

impl From<MinigrepArgsError> for MinigrepError {
    fn from(error: MinigrepArgsError) -> Self {
        MinigrepError::BadArgs(error)
    }
}

impl fmt::Debug for MinigrepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadArgs(args_error) => write!(f, "{args_error:?}"),
            Self::IoError(io_error) => write!(f, "{io_error}"),
            Self::NoResults => write!(f, "No results found."),
        }
    }
}

pub enum MinigrepArgsError {
    QueryMissing,
    PathMissing,
    QueryWhitespace,
    PathInaccessible(PathBuf),
}

impl fmt::Debug for MinigrepArgsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryMissing => write!(f, "Provide a search query."),
            Self::PathMissing => write!(f, "Provide a file path."),
            Self::QueryWhitespace => write!(f, "Search query cannot include whitespace."),
            Self::PathInaccessible(path) => {
                write!(f, "Could not access path '{}'.", path.display())
            }
        }
    }
}
