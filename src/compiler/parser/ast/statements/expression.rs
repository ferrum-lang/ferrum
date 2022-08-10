use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    FunctionCall(FunctionCall),
    MethodCall(MethodCall),
    Construction(Construction), // Example { a, b }
    Reference(Reference),    // example, some.example, some::example
    Loop(Loop),   // loop, loop-while, while, for
    Branch(Branch),       // if/else, match, ternary
    Operation(Operation),    // 1 + 2, 1 >= 2
    Closure(Closure),      // () => {}
    Literal(Literal),      // 1, "hello"
    Range(Range),        // 1..=10
    Block(BlockExpr),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCall {
    pub reciever: Option<Type>,
    pub name: String,
    pub args: Vec<FunctionCallArg>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCallArg {
    pub name: Option<String>,
    pub value: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MethodCall {
    pub reciever: Box<Expression>,
    pub name: String,
    pub args: Vec<FunctionCallArg>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Construction {
    pub r#type: Type,
    pub fields: Vec<ConstructionField>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConstructionField {
    Assign(ConstructionFieldAssign),
    Spread(ConstructionFieldSpread),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConstructionFieldAssign {
    pub name: String,
    pub value: Option<Box<Expression>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConstructionFieldSpread {
    pub value: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Reference {
    Static(ReferenceStatic),
    Instance(ReferenceInstance),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReferenceStatic {
    pub reciever: Type,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReferenceInstance {
    pub reciever: Option<Box<Expression>>,
    pub name: String,
}

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
    pub condition: Box<Expression>,
    pub arms: Vec<BranchMatchArm>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BranchMatchArm {
    pub pattern: Pattern,
    pub guard: Option<BranchMatchGuard>,
    pub body: Box<Block>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BranchMatchGuard {}

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    Literal(PatternLiteral),
    Struct(PatternStruct),
    TupleStruct(PatternTupleStruct),
    Tuple(PatternTuple),
    List(PatternList),
    Identity(PatternIdentity),
    Wild(PatternWild),
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternLiteral {}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternStruct {}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternTupleStruct {}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternTuple {}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternList {}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternIdentity {}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternWild {}

#[derive(Clone, Debug, PartialEq)]
pub struct BranchTernary {
    pub condition: Box<Expression>,
    pub if_then: Box<Expression>,
    pub else_then: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Operation {}

#[derive(Clone, Debug, PartialEq)]
pub struct Closure {}

#[derive(Clone, Debug, PartialEq)]
pub struct Literal {}

#[derive(Clone, Debug, PartialEq)]
pub struct Range {}

#[derive(Clone, Debug, PartialEq)]
pub struct BlockExpr {
    block: Box<Block>,
}

