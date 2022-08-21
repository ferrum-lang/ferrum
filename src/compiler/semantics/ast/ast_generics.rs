use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Generics {
}

#[derive(Clone, Debug, PartialEq)]
pub enum GenericParam {
    Type(TypeParam),
    Lifetime(LifetimeDef),
    Const(ConstParam),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeParam {
    pub ident: String,
    pub bounds: Vec<TypeParamBound>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeParamBound {
    Trait(TraitBound),
    Lifetime(Lifetime),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TraitBound {
    pub path: Path,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LifetimeDef {
    pub lifetime: Lifetime,
    pub bounds: Vec<Lifetime>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConstParam {
    pub ident: String,
    pub typ: Type,
    pub default: Option<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WhereClause {
    pub predicates: Vec<WherePredicate>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WherePredicate {
    Type(PredicateType),
    Lifetime(PredicateLifetime),
    Eq(PredicateEq),
}

#[derive(Clone, Debug, PartialEq)]
pub struct PredicateType {
    pub bounded_type: Type,
    pub bounds: Vec<TypeParamBound>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PredicateLifetime {
    pub lifetime: Lifetime,
    pub bounds: Vec<Lifetime>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PredicateEq {
    pub lhs_type: Type,
    pub rhs_type: Type,
}

