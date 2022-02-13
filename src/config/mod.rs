mod args;

use super::error::Error;
use args::{parse_env_args, ConfigArgs};
use std::path::PathBuf;

const RUST_FILE_EXT: &'static str = "rs";

#[derive(Debug, Clone)]
pub struct Config {
    pub input_filepath: PathBuf,
    pub output_filepath: PathBuf,
    pub build_dir: PathBuf,
    pub build_filename: String,
}

pub fn build_env_config<'a>() -> Result<Config, Error> {
    let args = parse_env_args();
    let config = build_config_from_args(args);

    return Ok(config);
}

pub fn build_config_from_args<'a>(args: ConfigArgs) -> Config {
    return Config {
        input_filepath: args.input_filepath,
        output_filepath: args.output_directory.join(&args.name),
        build_dir: args.output_directory.join("build"),
        build_filename: format!("{}.{}", &args.name, RUST_FILE_EXT),
    };
}
