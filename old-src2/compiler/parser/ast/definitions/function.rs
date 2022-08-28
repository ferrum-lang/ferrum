use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct DefFn {
    pub signature: DefFnSignature,
    pub r#impl: DefFnImpl,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefFnSignature {
    pub is_public: bool,
    pub is_async: MaybeBool,
    pub name: String,
    pub generics: Option<DefGenerics>,
    pub params: Vec<DefFnParam>,
    pub return_type: Option<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefFnParam {
    pub is_mutable: bool,
    pub is_var_args: bool,
    pub name: String,
    pub r#type: Type,
    pub default: Option<Box<Expression>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DefFnImpl {
    Expression(Expression),
    Block(Block),
}

