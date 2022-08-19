use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    Box(PatBox),
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
pub struct PatBox {
    pub pattern: Box<Pattern>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatLit {
    pub expr: Box<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatOr {
    pub cases: Vec<Box<Pattern>>,
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
    pub elems: Vec<Box<Pattern>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatStruct {
    pub path: Path,
    pub fields: Vec<FieldPat>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldPat {
    pub name: Member,
    pub pattern: Box<Pattern>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Member {
    Named(String),
    Indexed(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatTuple {
    pub elems: Vec<Box<Pattern>>,
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

