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

