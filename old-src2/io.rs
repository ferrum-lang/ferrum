use crate::config::Config;

use std::{fs, path};

use anyhow::{Result, Context};

pub fn read_file_contents(filepath: &path::PathBuf) -> Result<String> {
    let contents = fs::read_to_string(filepath)?;
    return Ok(contents);
}

pub fn write_to_build_dir(config: &Config, contents: String) -> Result<()> {
    let build_file = config.build_dir.join(&config.build_filename);

    fs::write(build_file.clone(), contents)
        .with_context(|| format!("{build_file:?}"))?;

    return Ok(());
}

