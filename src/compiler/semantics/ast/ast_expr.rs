#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Array(ExprArray),
    Assign(ExprAssign),
    AssignOp(ExprAssignOp),
    Async(ExprAsync),
    Await(ExprAwait),
    Binary(ExprBinary),
    Block(ExprBlock),
    Break(ExprBreak),
    Call(ExprCall),
    Cast(ExprCast),
    Closure(ExprClosure),
    Continue(ExprContinue),
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
    Yield(ExprYield),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprArray {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprAssign {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprAssignOp {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprAsync {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprAwait {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprBinary {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprBlock {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprBreak {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprCall {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprCast {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprClosure {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprContinue {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprField {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprForLoop {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprIf {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprIndex {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprLet {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprLit {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprLoop {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprMacro {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprMatch {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprMethodCall {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprParen {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprPath {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprRange {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprReference {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprReturn {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprStruct {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprTry {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprTuple {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprType {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprUnary {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprWhile {
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprYield {
}

