use super::*;

use super::super::ast::{self, *};

use super::super::super::lexer::{self, *};

use anyhow::Result;

pub fn parse_imports(
    ast: &mut AST,
    tokens: &mut Stack<TokenData>,
) -> Result<()> {
    while let Some(token) = tokens.peek() {
        match token.value {
            Token::Keyword(Keyword::Import) => {},
            _ => return Ok(()),
        };

        let import = build_import(tokens)?;
        ast.imports.push(import);

    }

    return Ok(());
}

fn build_import(tokens: &mut Stack<TokenData>) -> Result<Import> {
    match tokens.pop() {
        Some(TokenData { value: Token::Keyword(Keyword::Import), .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Keyword(Keyword::Import))))?,
    }

    let assignment = build_import_assignment(tokens)?;

    match tokens.pop() {
        Some(TokenData { value: Token::Keyword(Keyword::From), .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Keyword(Keyword::From))))?,
    };

    let source = match tokens.pop() {
        Some(TokenData { value: Token::Literal(lexer::Literal::PlainString(source)),  .. }) => source.to_string(),
        Some(token) => Err(ParseError::UnexpectedToken(token.clone()))?,
        _ => Err(ParseError::MissingExpectedToken(Some(Token::Literal(lexer::Literal::PlainString("some/import/source".to_string())))))?,
    };

    return Ok(ast::Import { assignment, source });
}

fn build_import_assignment(tokens: &mut Stack<TokenData>) -> Result<ImportAssignment> {
    let import_assignment = match tokens.pop() {
        Some(TokenData { value: Token::Identifier(identifier),  .. }) => ImportAssignment::Direct(ImportAssignmentDirect { name: identifier.to_string() }),
        Some(TokenData { value: Token::OpenBrace,  .. }) => {
            let assignment = ImportAssignment::Destructured(ImportAssignmentDestruct {
                items: vec![],
            });

            todo!("{assignment:?}");
        },
        Some(token) => Err(ParseError::UnexpectedToken(token.clone()))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier("some_identifier".to_string()))))?,
    };

    return Ok(import_assignment);
}

