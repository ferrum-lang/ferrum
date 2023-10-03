#![allow(clippy::needless_return)]

use ferrum_compiler::helpers;
use ferrum_compiler::result::Result;

use std::env;

use env_logger;

fn main() -> Result {
    env_logger::init();

    let target_dir = env::args().nth(1).unwrap_or_else(|| String::from("."));

    let out = helpers::run_full(target_dir.into())?;

    print!("{}", String::from_utf8(out.stdout)?);

    return Ok(());
}
