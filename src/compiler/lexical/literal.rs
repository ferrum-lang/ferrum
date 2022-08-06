use crate::utils::stack::Stack;

use super::source_meta::SourceMeta;

#[derive(Clone, Debug)]
pub struct Literals {
    pub lines: Vec<Vec<LiteralData>>,
}

#[derive(Clone, Debug)]
pub struct LiteralData {
    value: Literal,
    source_meta: SourceMeta,
}

#[derive(Clone, Debug)]
pub enum Literal {
    BlockComment,
    LineComment,

    // String value in string/char literals excludes the surrounding quotes & brackets,
    // but the literal's representation is inclusive of quotes and brackets
    PlainString(String),
    TemplateStringStart(String),
    TemplateStringMiddle(String),
    TemplateStringEnd(String),
    Char(String),

    // NewLine,               // \n
    Spacing(String),    //
    Number(String),     // 12345
    Identifier(String), // some_identifier_1

    OpeningBracket,     // }
    ClosingBracket,     // {
    OpeningBrace,       // [
    ClosingBrace,       // ]
    OpeningParenthesis, // (
    ClosingParenthesis, // )
    Comma,              // ,
    Semicolon,          // ;
    Colon,              // :
    Tilde,              // ~
    QuestionMark,       // ?
    Backslash,          // \
    Period,             // .
    Equals,             // =
    ExclamationMark,    // !
    Ampersand,          // &
    Pipe,               // |
    LessThan,           // <
    GreaterThan,        // >
    Plus,               // +
    Minus,              // -
    Asterisk,           // *
    ForwardSlash,       // /
    Percent,            // %
    Dollar,             // $
    Caret,              // ^
    At,                 // @

    Other(char),
}

pub fn parse_literals(contents: String) -> Literals {
    let mut current_line = vec![];
    let mut lines = vec![];

    let chars_vec = contents.chars().collect::<Vec<char>>();
    let mut chars = Stack::from_top_to_bottom_vec(chars_vec);

    let mut is_in_strings_template = false;

    while let Some(c) = chars.pop() {
        // TODO
        let source_meta = SourceMeta {
            filepath: std::path::PathBuf::from("todo"),
            lines: (0, 0),
        };

        match c {
            '/' if chars.peek() == Some(&'*') => {
                chars.pop();

                while let Some(c) = chars.pop() {
                    if c == '*' && chars.peek() == Some(&'/') {
                        chars.pop();
                        break;
                    }

                    chars.pop();
                }

                current_line.push(LiteralData {
                    value: Literal::BlockComment,
                    source_meta,
                });
            }
            '/' if chars.peek() == Some(&'/') => {
                chars.pop();

                while chars.peek() != Some(&'\n') {
                    chars.pop();
                }

                // Note: Literal::LineComment's representation doesn't include the new-line char
                // ending the comment

                current_line.push(LiteralData {
                    value: Literal::LineComment,
                    source_meta,
                });
            }

            '"' => {
                // Note: Need to account for both plain and template strings
                // Also accounting for escaped chars w/ backslash
                let mut value = String::new();

                let mut is_template = false;

                while let Some(c) = chars.pop() {
                    if c == '\\' {
                        value.push(c);

                        if let Some(c) = chars.pop() {
                            value.push(c);
                        }
                    } else {
                        match c {
                            '"' => {
                                is_template = false;
                                break;
                            }
                            '{' => {
                                is_template = true;
                                break;
                            }
                            _ => value.push(c),
                        }
                    }
                }

                if is_template {
                    is_in_strings_template = true;
                    current_line.push(LiteralData {
                        value: Literal::TemplateStringStart(value),
                        source_meta,
                    });
                } else {
                    current_line.push(LiteralData {
                        value: Literal::PlainString(value),
                        source_meta,
                    });
                }
            }
            '}' if is_in_strings_template => {
                let mut value = String::new();

                let mut is_template_middle = false;

                while let Some(c) = chars.pop() {
                    if c == '\\' {
                        value.push(c);

                        if let Some(c) = chars.pop() {
                            value.push(c);
                        }
                    } else {
                        match c {
                            '"' => {
                                is_template_middle = false;
                                break;
                            }
                            '{' => {
                                is_template_middle = true;
                                break;
                            }
                            _ => value.push(c),
                        }
                    }
                }

                if is_template_middle {
                    current_line.push(LiteralData {
                        value: Literal::TemplateStringMiddle(value),
                        source_meta,
                    });
                } else {
                    is_in_strings_template = false;
                    current_line.push(LiteralData {
                        value: Literal::TemplateStringEnd(value),
                        source_meta,
                    });
                }
            }

            '\'' => {
                let mut value = String::new();

                while let Some(c) = chars.pop() {
                    if c == '\\' {
                        value.push(c);

                        if let Some(c) = chars.pop() {
                            value.push(c);
                        }
                    } else {
                        match c {
                            '\'' => break,
                            _ => value.push(c),
                        }
                    }
                }

                current_line.push(LiteralData {
                    value: Literal::Char(value),
                    source_meta,
                });
            }

            '\n' => {
                if !chars.is_empty() {
                    lines.push(current_line);
                    current_line = vec![];
                }

                // current_line.push(LiteralData { value: Literal::NewLine, source_meta });
            }

            _ if c.is_whitespace() => current_line.push(LiteralData {
                value: parse_buffered_literal(
                    &mut chars,
                    c,
                    Box::new(|_, peek| peek.is_whitespace() && peek != '\n'),
                    Box::new(Literal::Spacing),
                ),
                source_meta,
            }),
            _ if c.is_numeric() => current_line.push(LiteralData {
                value: parse_buffered_literal(
                    &mut chars,
                    c,
                    Box::new(|_, peek| peek.is_numeric()),
                    Box::new(Literal::Number),
                ),
                source_meta,
            }),
            _ if c.is_alphabetic() => current_line.push(LiteralData {
                value: parse_buffered_literal(
                    &mut chars,
                    c,
                    Box::new(|_, peek| peek.is_alphanumeric() || peek == '_'),
                    Box::new(Literal::Identifier),
                ),
                source_meta,
            }),

            '{' => current_line.push(LiteralData { value: Literal::OpeningBracket, source_meta }),
            '}' => current_line.push(LiteralData { value: Literal::ClosingBracket, source_meta }),
            '[' => current_line.push(LiteralData { value: Literal::OpeningBrace, source_meta }),
            ']' => current_line.push(LiteralData { value: Literal::ClosingBrace, source_meta }),
            '(' => current_line.push(LiteralData { value: Literal::OpeningParenthesis, source_meta }),
            ')' => current_line.push(LiteralData { value: Literal::ClosingParenthesis, source_meta }),
            ',' => current_line.push(LiteralData { value: Literal::Comma, source_meta }),
            ';' => current_line.push(LiteralData { value: Literal::Semicolon, source_meta }),
            ':' => current_line.push(LiteralData { value: Literal::Colon, source_meta }),
            '~' => current_line.push(LiteralData { value: Literal::Tilde, source_meta }),
            '?' => current_line.push(LiteralData { value: Literal::QuestionMark, source_meta }),
            '\\' => current_line.push(LiteralData { value: Literal::Backslash, source_meta }),
            '.' => current_line.push(LiteralData { value: Literal::Period, source_meta }),
            '=' => current_line.push(LiteralData { value: Literal::Equals, source_meta }),
            '!' => current_line.push(LiteralData { value: Literal::ExclamationMark, source_meta }),
            '&' => current_line.push(LiteralData { value: Literal::Ampersand, source_meta }),
            '|' => current_line.push(LiteralData { value: Literal::Pipe, source_meta }),
            '<' => current_line.push(LiteralData { value: Literal::LessThan, source_meta }),
            '>' => current_line.push(LiteralData { value: Literal::GreaterThan, source_meta }),
            '+' => current_line.push(LiteralData { value: Literal::Plus, source_meta }),
            '-' => current_line.push(LiteralData { value: Literal::Minus, source_meta }),
            '*' => current_line.push(LiteralData { value: Literal::Asterisk, source_meta }),
            '/' => current_line.push(LiteralData { value: Literal::ForwardSlash, source_meta }),
            '%' => current_line.push(LiteralData { value: Literal::Percent, source_meta }),
            '$' => current_line.push(LiteralData { value: Literal::Dollar, source_meta }),
            '^' => current_line.push(LiteralData { value: Literal::Caret, source_meta }),
            '@' => current_line.push(LiteralData { value: Literal::At, source_meta }),

            _ => current_line.push(LiteralData { value: Literal::Other(c), source_meta }),
        };
    }

    lines.push(current_line);

    return Literals { lines };
}

fn parse_buffered_literal(
    chars: &mut Stack<char>,
    c: char,
    predicate: Box<dyn Fn(&String, char) -> bool>,
    literal_factory: Box<dyn Fn(String) -> Literal>,
) -> Literal {
    let mut buffer = c.to_string();

    while let Some(&peek) = chars.peek() {
        if !predicate(&buffer, peek) {
            break;
        }

        buffer.push(peek);
        chars.pop();
    }

    return literal_factory(buffer);
}
