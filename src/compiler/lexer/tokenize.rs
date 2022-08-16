use crate::io;
use crate::utils::stack::Stack;

use super::error::LexicalError;
use super::source_meta::SourceMeta;
use super::tokens::{BuiltInType, Keyword, Literal, Token, TokenData, Tokens};

use anyhow::Result;

pub fn tokenize(filepath: &std::path::PathBuf) -> Result<Tokens> {
    let source_meta = |start_line: usize, end_line: usize| SourceMeta {
        filepath: filepath.clone(),
        lines: (start_line, end_line),
    };

    let file_contents = io::read_file_contents(&filepath)?;

    let chars_vec = file_contents.chars().collect::<Vec<char>>();
    let mut chars = Stack::from_top_to_bottom_vec(chars_vec);

    let mut current_line = 1;
    let mut tokens = vec![];

    let mut is_in_strings_template = false;

    while let Some(c) = chars.pop() {
        match c {
            '/' if chars.peek() == Some(&'*') => {
                chars.pop();

                while let Some(c) = chars.pop() {
                    if c == '\n' {
                        current_line += 1;
                    } else if c == '*' && chars.peek() == Some(&'/') {
                        chars.pop();
                        break;
                    }
                }
            },
            '/' if chars.peek() == Some(&'/') => {
                chars.pop();

                while chars.peek() != Some(&'\n') {
                    chars.pop();
                }

                // Note: Literal::LineComment's representation doesn't include the new-line char
                // ending the comment
            },

            '"' => {
                // Note: Need to account for both plain and template strings
                // Also accounting for escaped chars w/ backslash
                let mut value = String::new();

                let start_line = current_line;

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
                    tokens.push(TokenData {
                        value: Token::Literal(Literal::TemplateStringStart(value)),
                        source_meta: source_meta(start_line, current_line),
                    });
                } else {
                    tokens.push(TokenData {
                        value: Token::Literal(Literal::PlainString(value)),
                        source_meta: source_meta(start_line, current_line),
                    });
                }
            },
            '}' if is_in_strings_template => {
                let mut value = String::new();

                let start_line = current_line;

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
                    tokens.push(TokenData {
                        value: Token::Literal(Literal::TemplateStringMiddle(value)),
                        source_meta: source_meta(start_line, current_line),
                    });
                } else {
                    is_in_strings_template = false;
                    tokens.push(TokenData {
                        value: Token::Literal(Literal::TemplateStringEnd(value)),
                        source_meta: source_meta(start_line, current_line),
                    });
                }
            },

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

                tokens.push(TokenData {
                    value: Token::Literal(Literal::Char(value)),
                    source_meta: source_meta(current_line, current_line),
                });
            },

            '\n' => {
                tokens.push(TokenData {
                    value: Token::NewLine,
                    source_meta: source_meta(current_line, current_line),
                });
                current_line += 1;
            },

            _ if c.is_whitespace() => {
                while let Some(&peek) = chars.peek() {
                    if !peek.is_whitespace() || peek == '\n' {
                        break;
                    }

                    chars.pop();
                }
            },
            _ if c.is_numeric() => {
                let mut buffer = c.to_string();

                let mut allow_period = true;
                let mut allow_e = true;
                let mut prev_was_period = false;

                while let Some(&peek) = chars.peek() {
                    if !peek.is_numeric() {
                        match (peek, allow_period, allow_e) {
                            ('.', false, _) => break,
                            ('e', _, false) => break,

                            ('.', _, _) => {
                                allow_period = false;
                                prev_was_period = true;
                            },
                            ('e', _, _) if !prev_was_period => {
                                allow_period = false;
                                allow_e = false;
                            },

                            _ => break,
                        }
                    } else {
                        if prev_was_period {
                            buffer.push('.');
                        }

                        prev_was_period = false;
                    }

                    if !prev_was_period {
                        buffer.push(peek);
                    }
                    chars.pop();
                }

                if prev_was_period {
                    chars.push('.');
                }

                tokens.push(TokenData {
                    value: Token::Literal(Literal::Number(buffer)),
                    source_meta: source_meta(current_line, current_line),
                });
            },
            _ if c.is_alphabetic() => {
                let mut buffer = c.to_string();

                while let Some(&peek) = chars.peek() {
                    if !peek.is_alphanumeric() && peek != '_' {
                        break;
                    }

                    buffer.push(peek);
                    chars.pop();
                }

                let token = match buffer.as_str() {
                    "true" => Token::Literal(Literal::Bool(true)),
                    "false" => Token::Literal(Literal::Bool(false)),
                    "some" => Token::Literal(Literal::Option { is_some: true }),
                    "none" => Token::Literal(Literal::Option { is_some: false }),
                    "ok" => Token::Literal(Literal::Result { is_ok: true }),
                    "err" => Token::Literal(Literal::Result { is_ok: false }),

                    "import" => Token::Keyword(Keyword::Import),
                    "from" => Token::Keyword(Keyword::From),
                    "pub" => Token::Keyword(Keyword::Pub),
                    "static" => Token::Keyword(Keyword::Static),
                    "fn" => Token::Keyword(Keyword::Fn),
                    "const" => Token::Keyword(Keyword::Const),
                    "let" => Token::Keyword(Keyword::Let),
                    "mut" => Token::Keyword(Keyword::Mut),
                    "struct" => Token::Keyword(Keyword::Struct),
                    "class" => Token::Keyword(Keyword::Class),
                    "interface" => Token::Keyword(Keyword::Interface),
                    "enum" => Token::Keyword(Keyword::Enum),
                    "errors" => Token::Keyword(Keyword::Errors),
                    "self" => Token::Keyword(Keyword::Self_),
                    "construct" => Token::Keyword(Keyword::Construct),
                    "impl" => Token::Keyword(Keyword::Impl),
                    "return" => Token::Keyword(Keyword::Return),
                    "yield" => Token::Keyword(Keyword::Yield),
                    "if" => Token::Keyword(Keyword::If),
                    "else" => Token::Keyword(Keyword::Else),
                    "match" => Token::Keyword(Keyword::Match),
                    "matches" => Token::Keyword(Keyword::Matches),
                    "loop" => Token::Keyword(Keyword::Loop),
                    "while" => Token::Keyword(Keyword::While),
                    "for" => Token::Keyword(Keyword::For),
                    "in" => Token::Keyword(Keyword::In),

                    "bool" => Token::BuiltInType(BuiltInType::Bool),
                    "bit" => Token::BuiltInType(BuiltInType::Bit),
                    "byte" => Token::BuiltInType(BuiltInType::Byte),
                    "uint" => Token::BuiltInType(BuiltInType::Uint),
                    "uint8" => Token::BuiltInType(BuiltInType::Uint8),
                    "uint16" => Token::BuiltInType(BuiltInType::Uint16),
                    "uint32" => Token::BuiltInType(BuiltInType::Uint32),
                    "uint64" => Token::BuiltInType(BuiltInType::Uint64),
                    "uint128" => Token::BuiltInType(BuiltInType::Uint128),
                    "biguint" => Token::BuiltInType(BuiltInType::BigUint),
                    "int" => Token::BuiltInType(BuiltInType::Int),
                    "int8" => Token::BuiltInType(BuiltInType::Int8),
                    "int16" => Token::BuiltInType(BuiltInType::Int16),
                    "int32" => Token::BuiltInType(BuiltInType::Int32),
                    "int64" => Token::BuiltInType(BuiltInType::Int64),
                    "int128" => Token::BuiltInType(BuiltInType::Int128),
                    "bigint" => Token::BuiltInType(BuiltInType::BigInt),
                    "float" => Token::BuiltInType(BuiltInType::Float),
                    "float32" => Token::BuiltInType(BuiltInType::Float32),
                    "float64" => Token::BuiltInType(BuiltInType::Float64),
                    "char" => Token::BuiltInType(BuiltInType::Char),
                    "string" => Token::BuiltInType(BuiltInType::String),

                    _ => Token::Identifier(buffer),
                };

                tokens.push(TokenData {
                    value: token,
                    source_meta: source_meta(current_line, current_line),
                });
            },

            '{' => tokens.push(TokenData {
                value: Token::OpenBrace,
                source_meta: source_meta(current_line, current_line),
            }),
            '}' => tokens.push(TokenData {
                value: Token::CloseBrace,
                source_meta: source_meta(current_line, current_line),
            }),
            '[' => tokens.push(TokenData {
                value: Token::OpenBracket,
                source_meta: source_meta(current_line, current_line),
            }),
            ']' => tokens.push(TokenData {
                value: Token::CloseBracket,
                source_meta: source_meta(current_line, current_line),
            }),
            '(' => tokens.push(TokenData {
                value: Token::OpenParenthesis,
                source_meta: source_meta(current_line, current_line),
            }),
            ')' => tokens.push(TokenData {
                value: Token::CloseParenthesis,
                source_meta: source_meta(current_line, current_line),
            }),
            ',' => tokens.push(TokenData {
                value: Token::Comma,
                source_meta: source_meta(current_line, current_line),
            }),
            ';' => tokens.push(TokenData {
                value: Token::Semicolon,
                source_meta: source_meta(current_line, current_line),
            }),

            ':' => {
                let token = match chars.peek() {
                    Some(&':') => {
                        chars.pop();
                        Token::DoubleColon
                    },
                    _ => Token::Colon,
                };

                tokens.push(TokenData {
                    value: token,
                    source_meta: source_meta(current_line, current_line),
                });
            },

            '~' => tokens.push(TokenData {
                value: Token::Tilde,
                source_meta: source_meta(current_line, current_line),
            }),

            '?' => {
                let token = match chars.peek() {
                    Some(&'?') => {
                        chars.pop();
                        Token::DoubleQuestionMark
                    },
                    _ => Token::QuestionMark,
                };

                tokens.push(TokenData {
                    value: token,
                    source_meta: source_meta(current_line, current_line),
                });
            },

            '.' => {
                let token = match chars.peek() {
                    Some(&'.') => {
                        chars.pop();
                        
                        match chars.peek() {
                            Some(&'=') => {
                                chars.pop();

                                Token::DoublePeriodEquals
                            },
                            _ => Token::DoublePeriod,
                        }
                    },
                    _ => Token::Period,
                };

                tokens.push(TokenData {
                    value: token,
                    source_meta: source_meta(current_line, current_line),
                });
            }

            '=' => {
                let token = match chars.peek() {
                    Some(&'=') => {
                        chars.pop();
                        Token::DoubleEquals
                    },
                    Some(&'>') => {
                        chars.pop();
                        Token::FatArrow
                    },
                    _ => Token::Equals,
                };

                tokens.push(TokenData {
                    value: token,
                    source_meta: source_meta(current_line, current_line),
                });
            }

            '!' => {
                let token = match chars.peek() {
                    Some(&'=') => {
                        chars.pop();
                        Token::NotEquals
                    },
                    _ => Token::ExclamationMark,
                };

                tokens.push(TokenData {
                    value: token,
                    source_meta: source_meta(current_line, current_line),
                });
            }

            '&' => {
                let token = match chars.peek() {
                    Some(&'&') => {
                        chars.pop();
                        Token::DoubleAmpersand
                    },
                    _ => Token::Ampersand,
                };

                tokens.push(TokenData {
                    value: token,
                    source_meta: source_meta(current_line, current_line),
                });
            }

            '|' => {
                let token = match chars.peek() {
                    Some(&'|') => {
                        chars.pop();
                        Token::DoublePipe
                    },
                    _ => Token::Pipe,
                };

                tokens.push(TokenData {
                    value: token,
                    source_meta: source_meta(current_line, current_line),
                });
            }

            '<' => {
                let token = match chars.peek() {
                    Some(&'=') => {
                        chars.pop();
                        Token::LessThanEquals
                    },
                    _ => Token::LessThan,
                };

                tokens.push(TokenData {
                    value: token,
                    source_meta: source_meta(current_line, current_line),
                });
            }

            '>' => {
                let token = match chars.peek() {
                    Some(&'=') => {
                        chars.pop();
                        Token::GreaterThanEquals
                    },
                    _ => Token::GreaterThan,
                };

                tokens.push(TokenData {
                    value: token,
                    source_meta: source_meta(current_line, current_line),
                });
            }

            '+' => {
                let token = match chars.peek() {
                    Some(&'=') => {
                        chars.pop();
                        Token::PlusEquals
                    },
                    _ => Token::Plus,
                };

                tokens.push(TokenData {
                    value: token,
                    source_meta: source_meta(current_line, current_line),
                });
            }

            '-' => {
                let token = match chars.peek() {
                    Some(&'=') => {
                        chars.pop();
                        Token::MinusEquals
                    },
                    Some(&'>') => {
                        chars.pop();
                        Token::SkinnyArrow
                    },
                    _ => Token::Minus,
                };

                tokens.push(TokenData {
                    value: token,
                    source_meta: source_meta(current_line, current_line),
                });
            }

            '*' => {
                let token = match chars.peek() {
                    Some(&'=') => {
                        chars.pop();
                        Token::AsteriskEquals
                    },
                    _ => Token::Asterisk,
                };

                tokens.push(TokenData {
                    value: token,
                    source_meta: source_meta(current_line, current_line),
                });
            }

            '/' => {
                let token = match chars.peek() {
                    Some(&'=') => {
                        chars.pop();
                        Token::ForwardSlashEquals
                    },
                    _ => Token::ForwardSlash,
                };

                tokens.push(TokenData {
                    value: token,
                    source_meta: source_meta(current_line, current_line),
                });
            }

            '%' => {
                let token = match chars.peek() {
                    Some(&'=') => {
                        chars.pop();
                        Token::PercentEquals
                    },
                    _ => Token::Percent,
                };

                tokens.push(TokenData {
                    value: token,
                    source_meta: source_meta(current_line, current_line),
                });
            }

            '^' => {
                let token = match chars.peek() {
                    Some(&'=') => {
                        chars.pop();
                        Token::CaretEquals
                    },
                    _ => Token::Caret,
                };

                tokens.push(TokenData {
                    value: token,
                    source_meta: source_meta(current_line, current_line),
                });
            }

            '$' => tokens.push(TokenData {
                value: Token::Dollar,
                source_meta: source_meta(current_line, current_line),
            }),
            '@' => tokens.push(TokenData {
                value: Token::At,
                source_meta: source_meta(current_line, current_line),
            }),

            _ => Err(LexicalError::UnexpectedCharacter { c, line: current_line })?,
        };
    }

    return Ok(Tokens { value: tokens });
}
