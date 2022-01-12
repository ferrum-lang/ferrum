use super::{config::Config, Error};
use std::fs;

pub fn read_input_contents(config: &Config) -> Result<String, Error> {
  return fs::read_to_string(&config.input_filepath).or_else(|e| Err(Error::new(e.to_string())));
}

pub fn write_to_build_dir(config: &Config, contents: String) -> Result<(), Error> {
  let build_file = config.build_dir.join(&config.build_filename);
  todo!("write_to_file: {:?}", build_file);
}
