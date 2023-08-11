use std::{fmt, fs, io};

mod minigrep_args;
use crate::minigrep_args::{MinigrepArgs, MinigrepArgsError};

enum MinigrepError {
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
            Self::BadArgs(minigrep_args_error) => write!(f, "{minigrep_args_error:?}"),
            Self::IoError(io_error) => write!(f, "{io_error}"),
            Self::NoResults => write!(f, "No results found."),
        }
    }
}

fn main() -> Result<(), MinigrepError> {
    let minigrep_args = MinigrepArgs::from_env_args()?;
    println!(
        "Running minigrep with query '{}' on file at path '{}'.",
        minigrep_args.query(),
        minigrep_args.path().display()
    );

    let contents = fs::read_to_string(minigrep_args.path())?;

    let index = match contents.find(minigrep_args.query()) {
        Some(index) => index,
        None => return Err(MinigrepError::NoResults),
    };

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

    println!("First result at line {line}: {result}");

    Ok(())
}
