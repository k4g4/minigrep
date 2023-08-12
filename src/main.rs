use std::{env, fs};

use minigrep::minigrep_args::MinigrepArgs;
use minigrep::minigrep_error::MinigrepError;

fn main() -> Result<(), MinigrepError> {
    let minigrep_args = MinigrepArgs::from_strings(&mut env::args().skip(1))?;
    
    let contents = fs::read_to_string(minigrep_args.path())?;
    let index = contents
        .find(minigrep_args.query())
        .ok_or(MinigrepError::NoResults)?;
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
