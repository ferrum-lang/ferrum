use super::source_meta::SourceMeta;

#[derive(Clone, Debug, PartialEq)]
pub struct Tokens {
    pub value: Vec<TokenData>,
}

impl std::fmt::Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut line = 0;

        for token in self.value.iter() {
            let line_n = token.source_meta.lines.0;

            if line != line_n {
                line = line_n;

                let new_line = if line == 1 { "" } else { "\n\n" };

                if token.value == Token::NewLine {
                    if let Err(e) = write!(f, "{new_line}{line_n}. ") {
                        return Err(e);
                    }
                } else {
                    if let Err(e) = write!(f, "{new_line}{line_n}. {:?}", token.value) {
                        return Err(e);
                    }
                }
            } else if token.value != Token::NewLine {
                if let Err(e) = write!(f, " {:?}", token.value) {
                    return Err(e);
                }
            }
        }

        return Ok(());
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TokenData {
    pub value: Token,
    pub source_meta: SourceMeta,
}

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

#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    Import,
    From,
    Pub,
    Fn,
    Const,
    Let,
    Mut,
    Struct,
    Class,
    Enum,
    Errors,
    Self_,
    Construct,
    Return,
    If,
    Else,
    Loop,
    While,
    For,
    In,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BuiltInType {
    Bool,
    Bit,
    Byte,
    Uint,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uint128,
    BigUint,
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    BigInt,
    Float,
    Float32,
    Float64,
    Char,
    String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    // String value in string/char literals excludes the surrounding quotes & brackets,
    // but the literal's representation is inclusive of quotes and brackets
    PlainString(String),

    TemplateStringStart(String),
    TemplateStringMiddle(String),
    TemplateStringEnd(String),

    Char(String),
    Number(String), // 12345
    Bool(bool),   // true | false
    Option { is_some: bool }, // none | some
    Result { is_ok: bool }, // ok | error
}

