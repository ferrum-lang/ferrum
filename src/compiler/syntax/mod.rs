#[derive(Debug, Clone)]
pub struct SyntaxTree {
    pub imports: Vec<ImportNode>,
    pub items: Vec<ItemNode>,
}

impl SyntaxTree {
    pub fn new() -> Self {
        Self {
            imports: vec![],
            items: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImportNode {
    pub assignment: ImportAssignmentNode,
    pub source_token: String,
}

#[derive(Debug, Clone)]
pub enum ImportAssignmentNode {
    Destructured(DestructureAssignmentNode),
}

#[derive(Debug, Clone)]
pub struct DestructureAssignmentNode {
    pub fields: Vec<DestructureAssignmentFieldNode>,
}

#[derive(Debug, Clone)]
pub struct DestructureAssignmentFieldNode {
    pub field_token: String,
    pub alias: Option<DestructureAssignmentFieldAliasNode>,
}

#[derive(Debug, Clone)]
pub struct DestructureAssignmentFieldAliasNode {
    pub name_token: String,
}

#[derive(Debug, Clone)]
pub enum ItemNode {
    Function(FunctionNode),
}

#[derive(Debug, Clone)]
pub struct FunctionNode {
    pub signature: FunctionSignatureNode,
    pub body: FunctionBodyNode,
}

#[derive(Debug, Clone)]
pub struct FunctionSignatureNode {
    pub is_public: bool,
    pub name_token: String,
    pub params: Vec<FunctionParamNode>,
    pub return_type: Option<ReturnTypeNode>,
}

#[derive(Debug, Clone)]
pub struct FunctionParamNode {
    pub name_token: String,
    pub is_mutable: bool,
    pub is_borrowed: bool,
    pub type_token: String,
}

#[derive(Debug, Clone)]
pub struct ReturnTypeNode {}

#[derive(Debug, Clone)]
pub struct FunctionBodyNode {
    pub statements: Vec<StatementNode>,
}

#[derive(Debug, Clone)]
pub enum StatementNode {
    Assignment(AssignmentNode),
    Expression(ExpressionNode),
}

#[derive(Debug, Clone)]
pub struct AssignmentNode {
    pub left: AssignmentLeftNode,
    pub right: ExpressionNode,
}

#[derive(Debug, Clone)]
pub struct AssignmentLeftNode {
    pub reassignable: bool,
    pub name_token: String,
    pub explicit_type: Option<TypeNode>,
}

#[derive(Debug, Clone)]
pub enum TypeNode {
    Primative(PrimativeTypeNode),
    Structure(StructureTypeNode),
    Contract(ContractTypeNode),
    Tuple(TupleTypeNode),
}

#[derive(Debug, Clone)]
pub struct PrimativeTypeNode {
    pub name_token: String,
    pub is_borrowed: bool,
    pub is_optional: bool,
}

#[derive(Debug, Clone)]
pub struct StructureTypeNode {
    pub path: StructureTypePathNode,
    pub generics: Option<Vec<TypeNode>>,
    pub is_mutable: bool,
    pub is_borrowed: bool,
    pub is_optional: bool,
}

#[derive(Debug, Clone)]
pub struct ContractTypeNode {
    pub path: StructureTypePathNode,
    pub is_mutable: bool,
    pub is_borrowed: bool,
    pub is_optional: bool,
}

#[derive(Debug, Clone)]
pub struct TupleTypeNode {
    pub segments: Vec<TypeNode>,
    pub is_optional: bool,
}

#[derive(Debug, Clone)]
pub enum ExpressionNode {
    Call(ExpressionCallNode),
    InstanceReference(InstanceReferenceNode),
    InstanceAccess(InstanceAccessNode),
    Literal(LiteralDataNode),
    Binary(BinaryExpressionNode),
    Range(RangeExpressionNode),
    Closure(ClosureExpressionNode),
    NullCoalesce(NullCoalesceExpressionNode),
    QuestionCast(QuestionCastNode),
}

#[derive(Debug, Clone)]
pub struct StructureTypePathNode {
    pub segments: Vec<StructureTypePathSegmentNode>,
}

#[derive(Debug, Clone)]
pub struct StructureTypePathSegmentNode {
    pub name_token: String,
}

#[derive(Debug, Clone)]
pub struct InstanceAccessNode {
    pub left: Box<ExpressionNode>,
    pub right: InstanceAccessRightNode,
}

#[derive(Debug, Clone)]
pub enum InstanceAccessRightNode {
    Access(Box<InstanceAccessRightNode>),
    Reference(InstanceReferenceNode),
    Call(ExpressionCallNode),
}

#[derive(Debug, Clone)]
pub struct ExpressionCallNode {
    pub subject: ExpressionCallPathNode,
    pub args: Vec<ExpressionNode>,
}

#[derive(Debug, Clone)]
pub struct ExpressionCallPathNode {
    pub segments: Vec<ExpressionCallPathSegmentNode>,
}

#[derive(Debug, Clone)]
pub enum ExpressionCallPathSegmentNode {
    TypeIdentity(String),
    FunctionIdentity(String),
}

#[derive(Debug, Clone)]
pub struct InstanceReferenceNode {
    pub name_token: String,
    pub is_borrowed: bool,
    pub is_mutable: bool,
}

#[derive(Debug, Clone)]
pub enum LiteralDataNode {
    Boolean(bool),
    Integer(String),
    Char(String),
    PlainString(String),
    TemplateString(TemplateStringNode),
    Tuple(TupleNode),
    List(ListNode),
}

#[derive(Debug, Clone)]
pub struct TemplateStringNode {
    pub start_token: String,
    pub middle_tokens: Vec<String>,
    pub expressions: Vec<ExpressionNode>,
    pub end_token: String,
}

#[derive(Debug, Clone)]
pub struct TupleNode {
    pub segments: Vec<ExpressionNode>,
}

#[derive(Debug, Clone)]
pub enum ListNode {
    Segmented(SegmentedListNode),
    ForIn(ForInListNode),
}

#[derive(Debug, Clone)]
pub struct SegmentedListNode {
    pub segments: Vec<ExpressionNode>,
}

#[derive(Debug, Clone)]
pub struct ForInListNode {
    pub expression: Box<ExpressionNode>,
    pub for_name_token: String,
    pub range: Box<ExpressionNode>,
}

#[derive(Debug, Clone)]
pub struct BinaryExpressionNode {
    pub left: Box<ExpressionNode>,
    pub right: Box<ExpressionNode>,
    pub op: BinaryOpNode,
}

#[derive(Debug, Clone)]
pub enum BinaryOpNode {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
    Modulo,
    Or,
    And,
    Eq,
    NotEq,
    Gt,
    GtOrEq,
    Lt,
    LtOrEq,
}

#[derive(Debug, Clone)]
pub struct RangeExpressionNode {
    pub left: Box<ExpressionNode>,
    pub right: Box<ExpressionNode>,
}

#[derive(Debug, Clone)]
pub struct NullCoalesceExpressionNode {
    pub left: Box<ExpressionNode>,
    pub right: Box<ExpressionNode>,
}

#[derive(Debug, Clone)]
pub struct ClosureExpressionNode {
    pub signature: ClosureSignatureNode,
    pub body: FunctionBodyNode,
}

#[derive(Debug, Clone)]
pub struct ClosureSignatureNode {
    pub params: Vec<ClosureParamNode>,
    pub return_type: Option<ReturnTypeNode>,
}

#[derive(Debug, Clone)]
pub struct ClosureParamNode {
    pub name_token: String,
    pub is_mutable: bool,
    pub is_borrowed: bool,
    pub type_token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct QuestionCastNode {
    pub expression: Box<ExpressionNode>,
}
