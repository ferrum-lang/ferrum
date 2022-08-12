#[derive(Clone, Debug, PartialEq)]
pub struct DefErrors {
  pub values: Vec<DefErrorsValue>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefErrorsValue {
    pub name: String,
}

