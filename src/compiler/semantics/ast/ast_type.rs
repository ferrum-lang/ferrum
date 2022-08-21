use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Array(TypeArray),
    BareFn(TypeBareFn),
    ImplTrait(TypeImplTrait),
    Infer,
    Path(TypePath),
    Reference(TypeReference),
    Slice(TypeSlice),
    TraitObject(TypeTraitObject),
    Tuple(TypeTuple),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeArray {
    pub elem: Box<Type>,
    pub len: Expr,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeBareFn {
    pub inputs: Vec<BareFnParam>,
    pub output: ReturnType,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BareFnParam {
    pub name: Option<String>,
    pub typ: Type,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeImplTrait {
    pub bounds: Vec<TypeParamBound>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypePath {
    pub path: Path,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeReference {
    pub lifetime: Option<Lifetime>,
    pub is_mut: bool,
    pub elem: Box<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeSlice {
    pub elem: Box<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeTraitObject {
    pub is_dyn: bool,
    pub bounds: Vec<TypeParamBound>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeTuple {
    pub elems: Vec<Type>,
}

