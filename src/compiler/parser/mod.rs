use super::{
  tokens::{ParsedToken, UnparsedToken},
  Error,
};

pub fn parse_tokens(unparsed_tokens: Vec<UnparsedToken>) -> Result<Vec<ParsedToken>, Error> {
  let mut tokens = vec![
    ParsedToken::Import,
    ParsedToken::Whitespace(" ".to_string()),
    ParsedToken::DestructureOpenBrace,
    ParsedToken::DestructureField("Console".to_string()),
    ParsedToken::DestructureCloseBrace,
    ParsedToken::Whitespace(" ".to_string()),
    ParsedToken::ImportFrom,
    ParsedToken::ImportSource("std".to_string()),
    ParsedToken::Semicolon,
    ParsedToken::Whitespace("\n\n".to_string()),
    ParsedToken::Function,
    ParsedToken::Whitespace(" ".to_string()),
    ParsedToken::FunctionName("main".to_string()),
    ParsedToken::FunctionParamsOpenParenthesis,
    ParsedToken::FunctionParamsCloseParenthesis,
    ParsedToken::Whitespace(" ".to_string()),
    ParsedToken::FunctionExpressionsOpenBrace,
    ParsedToken::Whitespace("\n".to_string()),
    ParsedToken::TypeName("Console".to_string()),
    ParsedToken::TypeAccessDoubleSemicolon,
    ParsedToken::AccessName("write_line".to_string()),
    ParsedToken::FunctionCallOpenParenthesis,
    ParsedToken::PlainString("Hello world!".to_string()),
    ParsedToken::FunctionCallCloseParenthesis,
    ParsedToken::Semicolon,
    ParsedToken::Whitespace("\n".to_string()),
    ParsedToken::FunctionExpressionsCloseBrace,
    ParsedToken::Whitespace("\n".to_string()),
  ];

  todo!("parse_tokens\nParsed tokens: {:?}", tokens);
}
