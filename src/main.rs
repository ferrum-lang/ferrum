use ferrum_oxidize::Result;

use std::path::PathBuf;

fn main() -> Result {
    let config = ferrum_oxidize::Config {
        entry_file: Some(PathBuf::from("./resources/main.fe")),
        build_dir: Some(PathBuf::from("./resources/.ferrum")),
        out_file: Some(PathBuf::from("./resources/main")),
        ..Default::default()
    };

    ferrum_oxidize::build_project(config)?;

    return Ok(());
}

