use syn::File;
use quote::ToTokens;

use anyhow::Result;

pub fn generate_rust(ast: File) -> Result<String> {
    return Ok(ast.into_token_stream().to_string());
}

