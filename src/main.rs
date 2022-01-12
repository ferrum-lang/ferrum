mod compiler;
mod config;
mod error;
mod executor;
mod io;

use error::Error;

fn main() -> Result<(), Error> {
    let config = config::build_env_config()?;

    let input_contents = io::read_file_contents(&config.input_filepath)?;

    let output_contents = compiler::compile_to_rust(input_contents)?;

    io::write_to_file(&config.output_filepath, output_contents)?;

    return executor::build_and_run_rust(&config.output_filepath);
}
