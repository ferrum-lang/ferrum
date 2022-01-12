mod compiler;
mod config;
mod error;
mod executor;
mod io;

use error::Error;

fn main() -> Result<(), Error> {
    let config = config::build_env_config()?;

    let input_contents = io::read_input_contents(&config)?;

    let output_contents = compiler::compile(input_contents)?;

    io::write_to_build_dir(&config, output_contents)?;

    return executor::build_and_run(&config);
}
