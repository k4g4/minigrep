use std::{fmt, io, path::PathBuf, str::Utf8Error};

pub enum MinigrepError {
    QueryMissing,
    UnknownArgument(String),
    PathInaccessible(PathBuf),
    IoError(io::Error),
    Utf8Error(Utf8Error),
    NoResults,
    Help, //not strictly an error, but works best here
}

impl From<io::Error> for MinigrepError {
    fn from(error: io::Error) -> Self {
        MinigrepError::IoError(error)
    }
}

impl From<Utf8Error> for MinigrepError {
    fn from(error: Utf8Error) -> Self {
        MinigrepError::Utf8Error(error)
    }
}

impl fmt::Debug for MinigrepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryMissing => write!(f, "Provide a search phrase. Use --help."),
            Self::UnknownArgument(arg) => write!(f, "Unknown argument '{arg}'"),
            Self::PathInaccessible(path) => {
                write!(f, "Could not access path '{}'.", path.display())
            }
            Self::IoError(error) => write!(f, "{error}"),
            Self::Utf8Error(error) => write!(f, "{error}"),
            Self::NoResults => write!(f, "No results found."),
            Self::Help => Ok(()),
        }
    }
}
