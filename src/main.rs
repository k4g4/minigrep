use std::env;

use minigrep::args::MinigrepArgs;
use minigrep::error::MinigrepError;
use minigrep::run;

fn main() -> Result<(), MinigrepError> {
    let args = MinigrepArgs::from_strings(&mut env::args().skip(1))?;
    let results = run(args)?;
    println!("{results}");

    Ok(())
}
