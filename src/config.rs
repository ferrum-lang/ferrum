use crate::args::Args;

use std::path::PathBuf;

const RUST_FILE_EXT: &'static str = "rs";

#[derive(Debug, Clone)]
pub struct Config {
    pub input_filepath: PathBuf,
    pub output_filepath: PathBuf,
    pub build_dir: PathBuf,
    pub build_filename: String,
}

pub fn build(args: Args) -> Config {
    return Config {
        input_filepath: args.input_filepath,
        output_filepath: args.output_directory.join(&args.name),
        build_dir: args.output_directory.join("build"),
        build_filename: format!("{}.{}", &args.name, RUST_FILE_EXT),
    };
}

