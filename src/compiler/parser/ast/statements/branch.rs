use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Branch {
    IfElse(BranchIfElse),
    Match(BranchMatch),
    Ternary(BranchTernary),
}

#[derive(Clone, Debug, PartialEq)]
pub struct BranchIf {
    pub condition: Box<Expression>,
    pub then: Box<Block>,
    pub r#else: Option<BranchIfElse>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BranchIfElse {
    pub condition: Box<Expression>,
    pub then: Box<Block>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BranchMatch {
    pub value: Box<Expression>,
    pub arms: Vec<BranchMatchArm>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BranchMatchArm {
    pub pattern: Pattern,
    pub guard: Option<Box<Expression>>,
    pub body: Box<Block>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BranchTernary {
    pub condition: Box<Expression>,
    pub if_then: Box<Expression>,
    pub else_then: Box<Expression>,
}

