use ferrum_oxidize::Result;

use std::path::PathBuf;

fn main() -> Result {
    let config = ferrum_oxidize::Config {
        entry_file: Some(PathBuf::from("./resources/src/_main.fe")),
        build_dir: Some(PathBuf::from("./resources/.ferrum/cargo_gen")),
        out_file: Some(PathBuf::from("./resources/main")),
        ..Default::default()
    };

    let project = ferrum_oxidize::build_project(config)?;

    let output = std::process::Command::new(project.out_file)
        .output()?;

    if !output.status.success() {
        let stderr = output.stderr;
        let string = String::from_utf8(stderr)?;

        panic!("{}", string);
    }

    let stdout = output.stdout;
    let string = String::from_utf8(stdout)?;

    println!("\n\n*** OUTPUT ***\n{}*** END ***", string);

    return Ok(());
}
