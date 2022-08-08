use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct DefFn {
    pub signature: DefFnSignature,
    pub body: DefFnImpl,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefFnSignature {
    pub is_public: bool,
    pub name: String,
    pub generics: Vec<Generic>,
    pub params: Vec<DefFnParam>,
    pub return_type: Option<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefFnParam {
    pub is_mutable: bool,
    pub name: String,
    pub alias: Option<String>,
    pub r#type: Type,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DefFnImpl {
    Direct(Expression),
    Body(FnImplBody),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FnImplBody {
    pub statements: Vec<Statement>,
}

