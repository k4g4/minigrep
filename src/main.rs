use std::env;

use minigrep::args::MinigrepArgs;
use minigrep::error::MinigrepError;
use minigrep::run;
use minigrep::HELP;

fn main() -> Result<(), MinigrepError> {
    let env_args = env::args().skip(1);

    let args = match MinigrepArgs::from_env_args(env_args) {
        Ok(args) => args,
        Err(err) => match err {
            MinigrepError::Help => {
                print_help();
                return Ok(());
            }
            err => {
                return Err(err);
            }
        },
    };

    print!("{:?}", run(args)?);

    Ok(())
}

fn print_help() {
    print!("{HELP}");
}
