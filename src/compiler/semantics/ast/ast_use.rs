#[derive(Clone, Debug, PartialEq)]
pub struct Use {
    pub is_public: bool,
    pub tree: UseTree,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UseTree {
    Path(UsePath),
    Name(UseName),
    Group(UseGroup),
}

#[derive(Clone, Debug, PartialEq)]
pub struct UsePath {
    pub ident: String,
    pub tree: Box<UseTree>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UseName {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UseGroup {
    pub items: Vec<UseTree>,
}

