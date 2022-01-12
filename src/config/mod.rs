mod args;
mod defaults;

use super::error::Error;
use args::{parse_env_args, ConfigArgs};
use std::path::PathBuf;

use defaults::{
  DEFAULT_INPUT_FILE_DIR, DEFAULT_INPUT_FILE_NAME, DEFAULT_OUTPUT_FILE_DIR,
  DEFAULT_OUTPUT_FILE_NAME,
};

const LANG_FILE_EXT: &'static str = "lang";
const RUST_FILE_EXT: &'static str = "rs";

#[derive(Debug, Clone)]
pub struct Config {
  pub input_filepath: PathBuf,
  pub output_filepath: PathBuf,
}

pub fn build_env_config<'a>() -> Result<Config, Error> {
  let args = parse_env_args()?;
  let config = build_config_from_args(args);

  return Ok(config);
}

pub fn build_config_from_args<'a>(args: ConfigArgs) -> Config {
  return Config {
    input_filepath: args.input_filepath.unwrap_or_else(default_input_filepath),
    output_filepath: args.output_filepath.unwrap_or_else(default_output_filepath),
  };
}

fn default_input_filepath() -> PathBuf {
  let file_dir = DEFAULT_INPUT_FILE_DIR;
  let file_name = format!("{}.{}", DEFAULT_INPUT_FILE_NAME, LANG_FILE_EXT);

  return PathBuf::from(file_dir).join(file_name);
}

fn default_output_filepath() -> PathBuf {
  let file_dir = DEFAULT_OUTPUT_FILE_DIR;
  let file_name = format!("{}.{}", DEFAULT_OUTPUT_FILE_NAME, RUST_FILE_EXT);

  return PathBuf::from(file_dir).join(file_name);
}
