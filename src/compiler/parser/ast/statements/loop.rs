use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Loop {
    Loop(LoopLoop),
    While(WhileLoop),
    For(ForLoop),
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoopLoop {
    pub statements: Vec<Statement>,
    pub post_while_condition: Option<Condition>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WhileLoop {
    pub condition: Condition,
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForLoop {
    // TODO
}

