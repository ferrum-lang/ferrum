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

    let output_contents = compiler::compile(&config.input_filepath)?;

    io::write_to_build_dir(&config, output_contents)?;

    return executor::build_and_run(&config);
}

