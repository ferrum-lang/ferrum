
#[allow(dead_code)]
#[derive(Debug, Clone)]
enum LangStringValue {
  Slice(&'static str),
  Owned(String),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LangString {
  value: LangStringValue,
}

impl LangString {
  #[allow(dead_code)]
  pub fn from_slice(slice: &'static str) -> Self {
    Self { value: LangStringValue::Slice(slice) }
  }

  #[allow(dead_code)]
  pub fn from_owned(string: String) -> Self {
    Self { value: LangStringValue::Owned(string) }
  }

  #[allow(dead_code)]
  pub fn as_slice(&self) -> &str {
    match &self.value {
      LangStringValue::Slice(x) => x,
      LangStringValue::Owned(x) => &*x,
    }
  }

  #[allow(dead_code)]
  pub fn as_owned(self) -> String {
    match self.value {
      LangStringValue::Slice(x) => x.to_string(),
      LangStringValue::Owned(x) => x,
    }
  }
}

impl PartialEq for LangString {
  fn eq(&self, other: &Self) -> bool {
    self.as_slice() == other.as_slice()
  }
}
impl Eq for LangString {}

impl std::hash::Hash for LangString {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
      self.as_slice().hash(state);
  }
}

impl std::fmt::Display for LangString {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    match &self.value {
      LangStringValue::Slice(x) => write!(f, "{}", x),
      LangStringValue::Owned(x) => write!(f, "{}", x),
    }
  }
}

// Placeholder for real BigInt lib
#[allow(dead_code)]
pub struct BigInt {}
impl BigInt {
  #[allow(dead_code)]
  pub fn new(_: i64) -> Self {
    Self {}
  }
}

// Placeholder for real BigUint lib
#[allow(dead_code)]
pub struct BigUint {}
impl BigUint {
  #[allow(dead_code)]
  pub fn new(_: u64) -> Self {
    Self {}
  }
}
