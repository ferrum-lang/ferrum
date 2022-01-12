use super::Error;
use std::{fs, path::Path};

pub fn read_file_contents<P: AsRef<Path>>(filepath: P) -> Result<String, Error> {
  return fs::read_to_string(filepath).or_else(|e| Err(Error::new(e.to_string())));
}

pub fn write_to_file<P: AsRef<Path>>(filepath: P, contents: String) -> Result<(), Error> {
  todo!("write_to_file");
}
