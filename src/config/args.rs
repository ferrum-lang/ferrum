use super::Error;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
  name = "Custom Lang Compiler",
  about = "Transpiles Lang code into Rust, then builds and executes the generated Rust code."
)]
pub struct ConfigArgs {
  /// Input filepath, or ./resources/main.lang by default
  #[structopt(
    short,
    long = "input",
    parse(from_os_str),
    default_value = "./resources/main.lang"
  )]
  pub input_filepath: PathBuf,

  /// Output directory, or ./out by default
  #[structopt(short = "d", long = "dir", parse(from_os_str), default_value = "./out")]
  pub output_directory: PathBuf,

  /// Output name, or main by default
  #[structopt(short, long, default_value = "main")]
  pub name: String,
}

pub fn parse_env_args() -> Result<ConfigArgs, Error> {
  return ConfigArgs::from_args_safe().or_else(|e| Err(Error::new(e.to_string())));
}
