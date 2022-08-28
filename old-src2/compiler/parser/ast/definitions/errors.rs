#[derive(Clone, Debug, PartialEq)]
pub struct DefErrors {
    pub is_public: bool,
    pub name: String,
    pub values: Vec<DefErrorsValue>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefErrorsValue {
    pub name: String,
}

