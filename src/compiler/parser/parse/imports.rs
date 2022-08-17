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
            let mut items = vec![];

            loop {
                ignore_new_lines(tokens);

                match tokens.peek() {
                    Some(TokenData { value: Token::CloseBrace, .. }) => {
                        tokens.pop();
                        break;
                    },
                    _ => {}
                }

                let item = build_import_destruct_item(tokens)?;
                items.push(item);

                ignore_new_lines(tokens);

                match tokens.pop() {
                    Some(TokenData { value: Token::CloseBrace, .. }) => break,
                    Some(TokenData { value: Token::Comma, .. }) => {},
                    Some(token) => Err(ParseError::UnexpectedToken(token))?,
                    None => Err(ParseError::MissingExpectedToken(Some(Token::CloseBrace)))?,
                }
            }

            ImportAssignment::Destructured(ImportAssignmentDestruct {
                items,
            })
        },
        Some(token) => Err(ParseError::UnexpectedToken(token.clone()))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier("some_identifier".to_string()))))?,
    };

    return Ok(import_assignment);
}

fn build_import_destruct_item(tokens: &mut Stack<TokenData>) -> Result<ImportAssignDestructItem> {
    let item = match tokens.pop() {
        Some(TokenData { value: Token::Identifier(ident), .. }) => {
            let alias = match tokens.peek() {
                Some(TokenData { value: Token::Colon, .. }) => match tokens.pop() {
                    Some(TokenData { value: Token::Identifier(alias), .. }) => Some(alias),
                    Some(token) => Err(ParseError::UnexpectedToken(token))?,
                    None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
                },
                _ => None,
            };

            ImportAssignDestructItem::Field(ImportAssignDestructField {
                name: ident,
                alias,
            })
        },
        Some(TokenData { value: Token::DoublePeriod, .. }) => {
            let name = match tokens.pop() {
                Some(TokenData { value: Token::Identifier(ident), .. }) => ident,
                Some(token) => Err(ParseError::UnexpectedToken(token))?,
                None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
            };

            ImportAssignDestructItem::Spread(ImportAssignDestructSpread {
                name,
            })
        },
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
    };

    return Ok(item);
}

