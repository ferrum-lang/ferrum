use ferrum_oxidize::Result;

use std::path::PathBuf;

fn main() -> Result {
    let config = ferrum_oxidize::Config {
        entry_file: Some(PathBuf::from("./resources/main.fe")),
        build_dir: Some(PathBuf::from("./resources/.ferrum/cargo_gen")),
        out_file: Some(PathBuf::from("./resources/main")),
        ..Default::default()
    };

    let project = ferrum_oxidize::build_project(config)?;

    let output = std::process::Command::new("cargo")
        .arg("run")
        .current_dir(project.build_dir)
        .output()?;

    if !output.status.success() {
        let stderr = output.stderr;
        let string = String::from_utf8(stderr)?;

        eprintln!("{}", string);

        todo!();
    }

    let stdout = output.stdout;
    let string = String::from_utf8(stdout)?;

    println!("{}", string);

    return Ok(());
}
