use std::env;

use minigrep::args::MinigrepArgs;
use minigrep::error::MinigrepError;
use minigrep::run;
use minigrep::HELP;

fn main() -> Result<(), MinigrepError> {
    let mut env_args = env::args().skip(1);

    let args = match MinigrepArgs::from_arg_strings(&mut env_args) {
        Ok(args) => args,
        Err(err) => match err {
            MinigrepError::Help => return Ok(print_help()),
            err => return Err(err),
        },
    };

    print!("{:?}", run(args)?);

    Ok(())
}

fn print_help() {
    print!("{HELP}");
}
