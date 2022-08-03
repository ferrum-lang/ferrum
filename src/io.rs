use crate::config::Config;

use std::fs;

use anyhow::Result;

pub fn read_input_contents(config: &Config) -> Result<String> {
    let contents = fs::read_to_string(&config.input_filepath)?;
    
    return Ok(contents);
}

pub fn write_to_build_dir(config: &Config, contents: String) -> Result<()> {
    let build_file = config.build_dir.join(&config.build_filename);

    fs::write(build_file, contents)?;

    return Ok(());
}

