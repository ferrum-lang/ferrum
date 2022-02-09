#[derive(Debug)]
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

#[derive(Debug)]
pub struct ImportNode {
  pub assignment: ImportAssignmentNode,
  pub source_token: String,
}

#[derive(Debug)]
pub enum ImportAssignmentNode {
  Destructured(DestructureAssignmentNode),
}

#[derive(Debug)]
pub struct DestructureAssignmentNode {
  pub fields: Vec<DestructureAssignmentFieldNode>,
}

#[derive(Debug)]
pub struct DestructureAssignmentFieldNode {
  pub field_token: String,
  pub alias: Option<DestructureAssignmentFieldAliasNode>,
}

#[derive(Debug)]
pub struct DestructureAssignmentFieldAliasNode {
  pub name_token: String,
}

#[derive(Debug)]
pub enum ItemNode {
  Function(FunctionNode),
}

#[derive(Debug)]
pub struct FunctionNode {
  pub signature: FunctionSignatureNode,
  pub body: FunctionBodyNode,
}

#[derive(Debug)]
pub struct FunctionSignatureNode {
  pub is_public: bool,
  pub name_token: String,
  pub params: Vec<FunctionParamNode>,
  pub return_type: Option<ReturnTypeNode>,
}

#[derive(Debug)]
pub struct FunctionParamNode {
  pub name_token: String,
  pub is_mutable: bool,
  pub is_borrowed: bool,
  pub type_token: String,
}

#[derive(Debug)]
pub struct ReturnTypeNode {}

#[derive(Debug)]
pub struct FunctionBodyNode {
  pub statements: Vec<StatementNode>,
}

#[derive(Debug)]
pub enum StatementNode {
  Assignment(AssignmentNode),
  Expression(ExpressionNode),
}

#[derive(Debug)]
pub struct AssignmentNode {
  pub left: AssignmentLeftNode,
  pub right: ExpressionNode,
}

#[derive(Debug)]
pub struct AssignmentLeftNode {
  pub reassignable: bool,
  pub name_token: String,
}

#[derive(Debug)]
pub enum ExpressionNode {
  Call(ExpressionCallNode),
  InstanceReference(InstanceReferenceNode),
  Literal(LiteralDataNode),
}

#[derive(Debug)]
pub struct ExpressionCallNode {
  pub subject: ExpressionCallPathNode,
  pub args: Vec<ExpressionNode>,
}

#[derive(Debug)]
pub struct ExpressionCallPathNode {
  pub segments: Vec<ExpressionCallPathSegmentNode>,
}

#[derive(Debug)]
pub enum ExpressionCallPathSegmentNode {
  TypeIdentity(String),
  FunctionIdentity(String),
}

#[derive(Debug)]
pub struct InstanceReferenceNode {
  pub name_token: String,
  pub is_borrowed: bool,
}

#[derive(Debug)]
pub enum LiteralDataNode {
  True,
  False,
  PlainString(String),
  TemplateString(TemplateStringNode),
  Integer(String),
}

#[derive(Debug)]
pub struct TemplateStringNode {
  pub start_token: String,
  pub middle_tokens: Vec<String>,
  pub expressions: Vec<ExpressionNode>,
  pub end_token: String,
}
