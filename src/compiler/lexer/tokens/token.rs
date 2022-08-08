use super::built_in::BuiltInType;
use super::keyword::Keyword;
use super::literal::Literal;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    NewLine, // \n

    Keyword(Keyword),
    BuiltInType(BuiltInType),
    Literal(Literal),

    Identifier(String), // some_identifier_1

    OpenBrace,        // }
    CloseBrace,       // {
    OpenBracket,      // [
    CloseBracket,     // ]
    OpenParenthesis,  // (
    CloseParenthesis, // )
    Comma,            // ,
    Semicolon,        // ;
    Colon,            // :
    Tilde,            // ~
    QuestionMark,     // ?
    Backslash,        // \
    Period,           // .
    Equals,           // =
    ExclamationMark,  // !
    Ampersand,        // &
    Pipe,             // |
    LessThan,         // <
    GreaterThan,      // >
    Plus,             // +
    Minus,            // -
    Asterisk,         // *
    ForwardSlash,     // /
    Percent,          // %
    Dollar,           // $
    Caret,            // ^
    At,               // @
}

