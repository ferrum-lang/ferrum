use ferrum_compiler::helpers;
use ferrum_compiler::result::Result;

use std::path::PathBuf;
// use std::process;

fn main() -> Result {
    let mut args = std::env::args().skip(1);

    let target_dir = args.next().unwrap_or_else(|| String::from("."));

    let target_dir = PathBuf::from(target_dir);

    // let verbose = match args.next() {
    //     Some(arg) if arg.as_str() == "-v" => true,
    //     _ => false,
    // };

    // TODO: support config in compiler
    // let config = Config {
    //     entry_file: Some(target_dir.join(PathBuf::from("src/_main.fe"))),
    //     build_dir: Some(target_dir.join(PathBuf::from(".ferrum/cargo_gen"))),
    //     out_file: Some(target_dir.join(PathBuf::from("main"))),
    //     verbose,
    //     ..Default::default()
    // };

    let out = helpers::run_full(target_dir)?;

    // let _ = process::Command::new("clear").status()?;

    // println!("{}", String::from_utf8(out.stderr)?);
    // println!("Output:\n------\n");

    print!("{}", String::from_utf8(out.stdout)?);

    return Ok(());
}
