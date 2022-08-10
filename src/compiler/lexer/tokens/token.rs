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

    OpenBrace,          // }
    CloseBrace,         // {
    OpenBracket,        // [
    CloseBracket,       // ]
    OpenParenthesis,    // (
    CloseParenthesis,   // )
    Comma,              // ,
    Semicolon,          // ;
    Colon,              // :
    DoubleColon,        // ::
    Tilde,              // ~
    QuestionMark,       // ?
    DoubleQuestionMark, // ??
    Period,             // .
    DoublePeriod,       // ..
    DoublePeriodEquals, // ..=
    Equals,             // =
    DoubleEquals,       // ==
    FatArrow,           // =>
    ExclamationMark,    // !
    NotEquals,          // !=
    Ampersand,          // &
    DoubleAmpersand,    // &&
    Pipe,               // |
    DoublePipe,         // ||
    LessThan,           // <
    LessThanEquals,     // <=
    GreaterThan,        // >
    GreaterThanEquals,  // >=
    Plus,               // +
    PlusEquals,         // +=
    Minus,              // -
    MinusEquals,        // -=
    SkinnyArrow,        // ->
    Asterisk,           // *
    AsteriskEquals,     // *=
    ForwardSlash,       // /
    ForwardSlashEquals, // /=
    Percent,            // %
    PercentEquals,      // %=
    Dollar,             // $
    Caret,              // ^
    At,                 // @
}

