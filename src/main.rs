#![allow(dead_code)]
use std::fs;

const IN_FILE: &'static str = "./resources/main.lang";
const OUT_FILE: &'static str = "./out/main.rs";

fn main() {
    let in_contents = fs::read_to_string(IN_FILE).expect(&*format!(
        "Something went wrong reading the file: {}",
        IN_FILE
    ));

    let tokens = parse_tokens_from_lang(in_contents);

    let out_contents = build_rust_from_tokens(tokens);

    fs::write(OUT_FILE, out_contents).expect(&*format!("Unable to write to file: {}", OUT_FILE));
}

fn parse_tokens_from_lang(contents: String) -> Vec<Token> {
    // let mut tokens: Vec<Token> = Vec::new();

    // let mut is_in_string = false;
    // let mut next_escaped;

    // let chars_iter = contents.chars();

    // for c in chars_iter {
    //     if !is_in_string {
    //         if c.is_whitespace() {
    //             println!("Whitespace.");
    //             continue;
    //         }

    //         if c == '"' {
    //             is_in_string = true;
    //             println!("STARTING STRING");
    //         }
    //     } else {
    //         if c == '\\' {
    //             next_escaped = true;
    //         } else {
    //             next_escaped = false;
    //         }

    //         if c == '"' && !next_escaped {
    //             is_in_string = false;
    //             println!("DONE STRING");
    //         }
    //     }

    //     println!("Char: \"{}\"", c);
    // }

    return vec![
        Token::Import(TokenImport {
            import_type: ImportType::Destructured(DestructuredImport {
                parts: vec!["Console".to_string()],
            }),
            source: "std".to_string(),
        }),
        Token::Function(TokenFunction {
            name: "main".to_string(),
            args: Vec::new(),
            is_public: false,
            return_type: None,
            tokens: vec![Token::Expression(TokenExpression::Action(
                ExpressionAction::TypeAccess(ActionTypeAccess {
                    type_name: "Console".to_string(),
                    access_type: AccessType::Action(Box::new(ExpressionAction::FunctionCall(
                        ActionFunctionCall {
                            name: "write_line".to_string(),
                            params: vec![FunctionCallParam::Data(Data::String(
                                "Hello world :)".to_string(),
                            ))],
                            has_semicolon: false,
                        },
                    ))),
                    has_semicolon: true,
                }),
            ))],
        }),
    ];
}

fn build_rust_from_tokens(tokens: Vec<Token>) -> String {
    let mut contents = "mod lang_prelude;\nuse lang_prelude::*;\n\n".to_string();

    for token in tokens {
        contents.push_str(&*format!("{}", token));
    }

    return contents;
}

impl std::fmt::Display for Token {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Token::Function(func) => write!(
                fmt,
                "{} fn {}({}) {} {{\n{}\n}}\n",
                if func.is_public { "pub" } else { "" },
                func.name,
                func.args
                    .iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<String>>()
                    .join(", "),
                if let Some(return_type) = &func.return_type {
                    format!("-> {}", return_type)
                } else {
                    String::new()
                },
                func.tokens
                    .iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<String>>()
                    .join("\n"),
            ),
            Token::Import(import) => write!(
                fmt,
                "mod {};\nuse {}::{{ {} }};\n",
                if import.source == "std".to_string() {
                    "lang_std".to_string()
                } else {
                    import.source.clone()
                },
                if import.source == "std".to_string() {
                    "lang_std".to_string()
                } else {
                    import.source.clone()
                },
                import.import_type,
            ),
            Token::Expression(exp) => {
                write!(fmt, "{}", exp)
            }
            _ => {
                todo!("{:?}", self);
            }
        }
    }
}

impl std::fmt::Display for FunctionArg {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            _ => todo!(),
        }
    }
}

impl std::fmt::Display for ImportType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            ImportType::Destructured(import) => write!(fmt, "{}", import.parts.join(", ")),
            _ => todo!(),
        }
    }
}

impl std::fmt::Display for TokenExpression {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            TokenExpression::Action(action) => write!(fmt, "{}", action),
            _ => todo!(),
        }
    }
}

impl std::fmt::Display for ExpressionAction {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            ExpressionAction::TypeAccess(access) => {
                write!(fmt, "{}::{}", access.type_name, access.access_type)
            }
            ExpressionAction::FunctionCall(func) => write!(
                fmt,
                "{}({}){}",
                func.name,
                func.params
                    .iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<String>>()
                    .join(", "),
                if func.has_semicolon { ";" } else { "" },
            ),
            _ => todo!(),
        }
    }
}

impl std::fmt::Display for AccessType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            AccessType::Action(action) => {
                write!(fmt, "{}", action)
            }
            _ => todo!(),
        }
    }
}

impl std::fmt::Display for FunctionCallParam {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            FunctionCallParam::Data(data) => {
                write!(fmt, "{}", data)
            }
            _ => todo!(),
        }
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Data::String(string) => write!(fmt, "LangString::from_slice(\"{}\")", string),
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
struct FunctionArgSelfType {}

#[derive(Debug)]
struct FunctionArgArg {}

#[derive(Debug)]
struct FunctionArgVarArgs {}

#[derive(Debug)]
enum FunctionArg {
    SelfType(FunctionArgSelfType),
    Arg(FunctionArgArg),
    VarArgs(FunctionArgVarArgs),
}

#[derive(Debug)]
enum AssignmentType {
    Const,
    Let,
}

#[derive(Debug)]
struct ExpressionAssignment {
    assignment_type: AssignmentType,
    name: String,
    data_type: Option<String>,
    data: Box<ExpressionAction>,
    has_semicolon: bool,
}

#[derive(Debug)]
struct ActionCondition {
    has_semicolon: bool,
}

#[derive(Debug)]
struct ParamVariable {
    name: String,
    is_borrowed: bool,
    is_mutable: bool,
}

#[derive(Debug)]
enum Data {
    String(String),
}

#[derive(Debug)]
enum FunctionCallParam {
    Variable(ParamVariable),
    Action(ExpressionAction),
    Data(Data),
}

#[derive(Debug)]
struct ActionFunctionCall {
    name: String,
    params: Vec<FunctionCallParam>,
    has_semicolon: bool,
}

#[derive(Debug)]
enum AccessType {
    Assignment(Box<ExpressionAssignment>),
    Action(Box<ExpressionAction>),
}

#[derive(Debug)]
struct ActionInstanceAccess {
    instance_name: String,
    access_type: AccessType,
    has_semicolon: bool,
}

#[derive(Debug)]
struct ActionTypeAccess {
    type_name: String,
    access_type: AccessType,
    has_semicolon: bool,
}

#[derive(Debug)]
enum ExpressionAction {
    Condition(ActionCondition),
    FunctionCall(ActionFunctionCall),
    InstanceAccess(ActionInstanceAccess),
    TypeAccess(ActionTypeAccess),
}

#[derive(Debug)]
struct ControlMatch {
    pattern: String,
    expected: String,
}

#[derive(Debug)]
struct ControlFor {}

#[derive(Debug)]
enum ExpressionControl {
    If(ActionCondition),
    Match(ControlMatch),
    Loop(),
    While(ActionCondition),
    For(ControlFor),
}

#[derive(Debug)]
enum TokenExpression {
    Assignment(ExpressionAssignment),
    Action(ExpressionAction),
    Control(ExpressionControl),
}

#[derive(Debug)]
struct TokenFunction {
    name: String,
    args: Vec<FunctionArg>,
    is_public: bool,
    return_type: Option<String>,
    tokens: Vec<Token>,
}

#[derive(Debug)]
struct TokenStructure {}

#[derive(Debug)]
struct TokenContract {}

#[derive(Debug)]
struct TokenEnum {}

#[derive(Debug)]
struct WholeImport {
    name: String,
}

#[derive(Debug)]
struct DestructuredImport {
    parts: Vec<String>,
}

#[derive(Debug)]
enum ImportType {
    Whole(WholeImport),
    Destructured(DestructuredImport),
}

#[derive(Debug)]
struct TokenImport {
    import_type: ImportType,
    source: String,
}

#[derive(Debug)]
enum Token {
    Function(TokenFunction),
    Structure(TokenStructure),
    Contract(TokenContract),
    Enum(TokenEnum),
    Import(TokenImport),
    Expression(TokenExpression),
}
