mod encoder;
mod parser;
mod types;

use encoder::build_rust_from_definitions;
use parser::parse_definitions_from_lang;
use std::fs;

const IN_FILE: &'static str = "./resources/main.lang";
const OUT_FILE: &'static str = "./out/main.rs";

fn main() -> Result<(), String> {
    let in_contents = fs::read_to_string(IN_FILE).or_else(|e| Err(e.to_string()))?;

    let definitions = parse_definitions_from_lang(in_contents)?;

    let out_contents = build_rust_from_definitions(definitions)?;

    fs::write(OUT_FILE, out_contents).or_else(|e| Err(e.to_string()))?;

    Ok(())
}
