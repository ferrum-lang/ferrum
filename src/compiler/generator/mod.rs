use syn::File;
use quote::ToTokens;

use anyhow::Result;

pub fn generate_rust(ast: File) -> Result<String> {
    todo!("\n\n{}\n\n", ast.into_token_stream())
}

