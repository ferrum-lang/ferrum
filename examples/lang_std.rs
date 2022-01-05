pub struct Console {}
impl Console {
  pub fn write_line<T: std::fmt::Display>(text: T) {
    println!("{}", text);
  }
}

pub struct StringBuilder {
  state: String,
}
impl StringBuilder {
  pub fn new() -> Self {
    Self { state: String::new() }
  }

  pub fn from(string: &str) -> Self {
    Self { state: string.to_string() }
  }

  pub fn append(mut self, string: &str) -> Self {
    self.state.push_str(string);
    return self;
  }

  pub fn build(self) -> String {
    self.state
  }
}

#[derive(Debug)]
pub struct Error<'a> {
  message: Option<&'a str>
}
impl<'a> Error<'a> {
  pub fn new(message: &'a str) -> Self {
    Self { message: Some(message) }
  }
  pub fn empty() -> Self {
    Self { message: None }
  }
}
impl<'a> std::fmt::Display for Error<'a> {
  fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    todo!()
  }
}
impl<'a> std::error::Error for Error<'a> {}

// Placeholder for real BigInt lib
pub struct BigInt {}
impl BigInt {
  pub fn new(_: i64) -> Self {
    Self {}
  }
}

// Placeholder for real BigUint lib
pub struct BigUint {}
impl BigUint {
  pub fn new(_: u64) -> Self {
    Self {}
  }
}
