use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    Ident(PatIdent),
    Lit(PatLit),
    Or(PatOr),
    Path(PatPath),
    Range(PatRange),
    Reference(PatReference),
    Slice(PatSlice),
    Struct(PatStruct),
    Tuple(PatTuple),
    TupleStruct(PatTupleStruct),
    Type(PatType),
    Rest,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatIdent {
    pub is_ref: bool,
    pub is_mut: bool,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatLit {
    pub expr: Box<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatOr {
    pub cases: Vec<Pattern>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatPath {
    pub path: Path,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Path {
    pub segments: Vec<PathSegment>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PathSegment {
    pub ident: String,
    pub arguments: PathArguments,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PathArguments {
    None,
    AngleBracketed(AngleBracketedGenericArguments),
    Parenthesized(ParenthesizedGenericArguments),
}

#[derive(Clone, Debug, PartialEq)]
pub struct AngleBracketedGenericArguments {
    pub args: Vec<GenericArgument>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum GenericArgument {
    Lifetime(Lifetime),
    Type(Type),
    Binding(Binding),
    Constraint(Constraint),
    Const(Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Binding {
    pub name: String,
    pub typ: Type,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Constraint {
    pub name: String,
    pub bounds: Vec<TypeParamBound>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParenthesizedGenericArguments {
    pub inputs: Vec<Type>,
    pub output: ReturnType,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatRange {
    pub inclusive_high: bool,
    pub low: Box<Expr>,
    pub high: Box<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatReference {
    pub is_mutable: bool,
    pub pattern: Box<Pattern>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatSlice {
    pub elems: Vec<Pattern>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatStruct {
    pub path: Path,
    pub fields: Vec<FieldPat>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldPat {
    pub member: Member,
    pub pattern: Option<Box<Pattern>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Member {
    Named(String),
    Indexed(u32),
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatTuple {
    pub elems: Vec<Pattern>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatTupleStruct {
    pub path: Path,
    pub pat_tuple: PatTuple,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatType {
    pub pattern: Box<Pattern>,
    pub typ: Box<Type>,
}

