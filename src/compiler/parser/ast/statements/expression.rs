use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    FunctionCall(FunctionCall),
    MethodCall(MethodCall),
    Construction(Construction), // Example { a, b }
    Reference(Reference),    // example, some.example, some::example
    Loop(Loop),   // loop, loop-while, while, for
    Branch(Branch),       // if/else, match, ternary
    BinaryOperation(BinaryOperation),    // 1 + 2, 1 >= 2
    Matches(Matches),
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
pub struct BinaryOperation {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operator: BinaryOperator,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Mod,
    Pow,

    And,
    Or,
    
    Equals,
    NotEquals,
    GreaterThan,
    GreatherThanOrEquals,
    LessThan,
    LessThanOrEquals,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Matches {
    pub value: Box<Expression>,
    pub pattern: Pattern,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Closure {
    pub signature: DefFnSignature,
    pub r#impl: Box<DefFnImpl>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClosureSignature {
    pub generics: Option<DefGenerics>,
    pub params: Vec<DefFnParam>,
    pub return_type: Option<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Number(LiteralNumber),
    String(LiteralString),
    Char(char),
    Bool(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralNumber {
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint128(u128),
    Uint(usize),
    // BigUint,

    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    Int(isize),
    // BigInt,

    Bit(bool),
    Byte(u8),

    Float32(f32),
    Float64(f64),
    Float(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralString {
    Plain(String),
    Template(TemplateString),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TemplateString {
    start: String,
    parts: Vec<TemplateStringPart>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TemplateStringPart {
    expression: Box<Expression>,
    post_string: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Range {
    from: LiteralNumber,
    to: LiteralNumber,
    inclusive: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlockExpr {
    block: Box<Block>,
}

