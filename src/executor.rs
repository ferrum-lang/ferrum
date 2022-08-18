use crate::config::Config;

use std::process::Command;

use anyhow::Result;

use thiserror::Error;

#[derive(Error, Debug)]
enum ExecutionError {
    #[error("Command failure")]
    CommandFailure(String),
}

pub fn build_and_run(config: &Config) -> Result<()> {
    let output = Command::new("rustc")
        .arg("-o")
        .arg(&config.output_filepath)
        .arg(&config.build_dir.join(&config.build_filename))
        .output()?;

    if !output.status.success() {
        let stderr = output.stderr;
        let string = String::from_utf8(stderr)?;

        eprintln!("{}", string);

        Err(ExecutionError::CommandFailure(string))?;
    }

    let output = Command::new(&config.output_filepath).output()?;

    let stdout = output.stdout;
    let string = String::from_utf8(stdout)?;

    print!("{}", string);

    return Ok(());
}

