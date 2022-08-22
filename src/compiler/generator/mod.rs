mod convert;
mod tokens;

use convert::convert_to_syn_ast;
pub use tokens::*;

use super::semantics::SemanticAST;

use quote::ToTokens;

pub fn generate_rust(sem_ast: SemanticAST) -> String {
    let syn_ast = convert_to_syn_ast(sem_ast);
    return syn_ast.into_token_stream().to_string();
}

