use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Array(ExprArray),
    Assign(ExprAssign),
    AssignOp(ExprAssignOp),
    Async(ExprAsync),
    Await(ExprAwait),
    Binary(ExprBinary),
    Block(Block),
    Break,
    Call(ExprCall),
    Cast(ExprCast),
    Closure(ExprClosure),
    Continue,
    Field(ExprField),
    ForLoop(ExprForLoop),
    If(ExprIf),
    Index(ExprIndex),
    Let(ExprLet),
    Lit(ExprLit),
    Loop(ExprLoop),
    Macro(ExprMacro),
    Match(ExprMatch),
    MethodCall(ExprMethodCall),
    Paren(ExprParen),
    Path(ExprPath),
    Range(ExprRange),
    Reference(ExprReference),
    // Repeat(ExprRepeat),
    Return(ExprReturn),
    Struct(ExprStruct),
    Try(ExprTry),
    Tuple(ExprTuple),
    Type(ExprType),
    Unary(ExprUnary),
    While(ExprWhile),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprArray {
    pub elems: Vec<Box<Expr>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprAssign {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprAssignOp {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub op: BinOp,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,
    // BitXor,
    // BitAnd,
    // BitOr,
    // Shl,
    // Shr,
    Eq,
    Lt,
    Le,
    Ne,
    Ge,
    Gt,
    AddEq,
    SubEq,
    MulEq,
    DivEq,
    RemEq,
    // BitXorEq,
    // BitAndEq,
    // BitOrEq,
    // ShlEq,
    // ShrEq,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprAsync {
    pub has_move: bool,
    pub block: Block,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprAwait {
    pub base: Box<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprBinary {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub op: BinOp,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprCall {
    pub func: Box<Expr>,
    pub args: Vec<Box<Expr>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprCast {
    pub expr: Box<Expr>,
    pub typ: Box<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprClosure {
    pub is_async: bool,
    pub has_move: bool,
    pub params: Vec<Pattern>,
    pub return_type: ReturnType,
    pub body: Box<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprField {
    pub base: Box<Expr>,
    pub member: Member,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprForLoop {
    pub pattern: Pattern,
    pub expr: Box<Expr>,
    pub body: Block,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprIf {
    pub condition: Box<Expr>,
    pub then_branch: Block,
    pub else_branch: Option<Block>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprIndex {
    pub expr: Box<Expr>,
    pub index: Box<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprLet {
    pub pattern: Pattern,
    pub expr: Box<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprLit {
    pub literal: Literal,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprLoop {
    pub body: Block,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprMacro {
    pub mac: Macro,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprMatch {
    pub expr: Box<Expr>,
    pub arms: Vec<MatchArm>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Box<Expr>>,
    pub body: Box<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprMethodCall {
    pub receiver: Box<Expr>,
    pub method: String,
    pub turbofish: Option<MethodTurbofish>,
    pub args: Vec<Box<Expr>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MethodTurbofish {
    pub args: Vec<GenericMethodArgument>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum GenericMethodArgument {
    Type(Type),
    Const(Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprParen {
    pub expr: Box<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprPath {
    pub path: Path,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprRange {
    pub to_inclusive: bool,
    pub from: Option<Box<Expr>>,
    pub to: Option<Box<Expr>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprReference {
    pub is_mutable: bool,
    pub expr: Box<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprReturn {
    pub expr: Option<Box<Expr>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprStruct {
    pub path: Path,
    pub fields: Vec<FieldValue>,
    pub rest: Option<Box<Expr>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldValue {
    pub member: Member,
    pub expr: Expr,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprTry {
    pub expr: Box<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprTuple {
    pub elems: Vec<Box<Expr>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprType {
    pub expr: Box<Expr>,
    pub typ: Box<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprUnary {
    pub expr: Box<Expr>,
    pub op: UnaryOp,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOp {
    Deref,
    Not,
    Neg,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprWhile {
    pub condition: Box<Expr>,
    pub body: Block,
}

