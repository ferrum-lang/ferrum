use super::{config::Config, error::Error};
use std::process::Command;

pub fn build_and_run(config: &Config) -> Result<(), Error> {
    Command::new("rustc")
        .arg("-o")
        .arg(&config.output_filepath)
        .arg(&config.build_dir.join(&config.build_filename))
        .output()
        .or_else(|e| Err(Error::new(e.to_string())))?;

    let output = Command::new(&config.output_filepath)
        .output()
        .or_else(|e| Err(Error::new(e.to_string())))?
        .stdout;

    let string = String::from_utf8(output).or_else(|e| Err(Error::new(e.to_string())))?;

    println!("{}", string);

    return Ok(());
}
