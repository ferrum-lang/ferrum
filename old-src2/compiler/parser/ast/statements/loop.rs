use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Loop {
    Loop(LoopLoop),
    While(LoopWhile),
    For(LoopFor),
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoopLoop {
    pub block: Box<Block>,
    pub post_while_condition: Option<Box<Expression>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoopWhile {
    pub condition: Box<Expression>,
    pub block: Box<Block>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoopFor {
    pub item: AssignmentTarget,
    pub expression: Box<Expression>,
    pub block: Box<Block>,
}

