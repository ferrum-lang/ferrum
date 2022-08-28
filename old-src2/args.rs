use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "Oxidize",
    about = "Compiler for the Ferrum programming language. Transpiles Ferrum code into Rust code; then builds and executes the generated Rust code."
)]
pub struct Args {
    /// Input filepath
    #[structopt(
        short,
        long = "input",
        parse(from_os_str),
        default_value = "./entry.fe"
    )]
    pub input_filepath: PathBuf,

    /// Output directory
    #[structopt(short = "d", long = "dir", parse(from_os_str), default_value = "./fe_build")]
    pub output_directory: PathBuf,

    /// Output name
    #[structopt(short, long, default_value = "main")]
    pub name: String,
}

pub fn parse_args() -> Args {
    return Args::from_args();
}

