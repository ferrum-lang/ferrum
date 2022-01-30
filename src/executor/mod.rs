use super::{config::Config, error::Error};
use std::process::Command;

pub fn build_and_run(config: &Config) -> Result<(), Error> {
    let output = Command::new("rustc")
        .arg("-o")
        .arg(&config.output_filepath)
        .arg(&config.build_dir.join(&config.build_filename))
        .output()
        .or_else(|e| Err(Error::new(e.to_string())))?;

    if !output.status.success() {
        let stderr = output.stderr;
        let string = String::from_utf8(stderr).or_else(|e| Err(Error::new(e.to_string())))?;

        eprintln!("{}", string);

        return Err(Error::new(string));
    }

    let output = Command::new(&config.output_filepath)
        .output()
        .or_else(|e| Err(Error::new(e.to_string())))?;

    let stdout = output.stdout;
    let string = String::from_utf8(stdout).or_else(|e| Err(Error::new(e.to_string())))?;

    println!("{}", string);

    return Ok(());
}
