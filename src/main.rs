mod args;
mod compiler;
mod config;
mod executor;
mod io;
mod utils;

use anyhow::Result;

fn main() -> Result<()> {
    let args = args::parse_args();
    
    let config = config::build(args);

    let input_contents = io::read_input_contents(&config)?;

    let output_contents = compiler::compile(input_contents)?;

    io::write_to_build_dir(&config, output_contents)?;

    return executor::build_and_run(&config);
}

