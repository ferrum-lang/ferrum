use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum List {
    Explicit(ListExplicit),
    FnFor(ListFnFor),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListExplicit {}

#[derive(Clone, Debug, PartialEq)]
pub struct ListFnFor {}

