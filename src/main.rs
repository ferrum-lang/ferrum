use ferrum_oxidize::Result;

use std::path::PathBuf;

fn main() -> Result {
    let mut args = std::env::args().skip(1);

    let target_dir = args.next().unwrap_or_else(|| String::from("."));

    let target_dir = PathBuf::from(target_dir);

    let verbose = match args.next() {
        Some(arg) if arg.as_str() == "-v" => true,
        _ => false,
    };

    let config = ferrum_oxidize::Config {
        entry_file: Some(target_dir.join(PathBuf::from("src/_main.fe"))),
        build_dir: Some(target_dir.join(PathBuf::from(".ferrum/cargo_gen"))),
        out_file: Some(target_dir.join(PathBuf::from("main"))),
        verbose,
        ..Default::default()
    };

    let project = ferrum_oxidize::build_project(config)?;

    let output = std::process::Command::new(project.out_file).output()?;

    if !output.status.success() {
        let stderr = output.stderr;
        let string = String::from_utf8(stderr)?;

        panic!("{}", string);
    }

    let stdout = output.stdout;
    let string = String::from_utf8(stdout)?;

    if verbose {
        println!("\n\n*** OUTPUT ***\n{}*** END ***", string);
    } else {
        println!("{string}");
    }

    return Ok(());
}
