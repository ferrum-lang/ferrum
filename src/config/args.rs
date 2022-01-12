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
  #[structopt(short, long = "input", parse(from_os_str))]
  pub input_filepath: Option<PathBuf>,

  /// Output filepath, or ./out/main by default
  #[structopt(short, long = "output", parse(from_os_str))]
  pub output_filepath: Option<PathBuf>,
}

pub fn parse_env_args() -> Result<ConfigArgs, Error> {
  return ConfigArgs::from_args_safe().or_else(|e| Err(Error::new(e.to_string())));
}
