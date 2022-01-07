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

// pub struct Shareable<T> {
//   value: T,
// }
// impl<T> Shareable<T> {

//   // Transpiler should decide which constructor based on whether `share` and `mutable` keywords are used
//   pub fn of_unique(value: T) -> Self {
//     Self { value, }
//   }

//   pub fn of_unique_mutable(value: T) -> Self {
//     Self { value, }
//   }
// }

// impl<T> std::ops::Deref for Shareable<T> {
//   type Target = T;

//   fn deref(&self) -> &Self::Target {
//     return &self.value;
//   }
// }

// impl<T> Shareable<std::rc::Rc<T>> {
//   pub fn of_shared(value: T) -> Self {
//     Self {
//       value: std::rc::Rc::new(value),
//     }
//   }

//   pub fn share(&self) -> Self {
//     let cloned = std::rc::Rc::clone(&self.value);

//     return Self {
//       value: cloned,
//     };
//   }
// }

// impl<T> Shareable<std::rc::Rc<std::cell::RefCell<T>>> {
//   pub fn of_shared_mutable(value: T) -> Self {
//     Self {
//       value: std::rc::Rc::new(std::cell::RefCell::new(value)),
//     }
//   }

//   pub fn borrow(&self) -> std::cell::Ref<T> {
//     return self.value.borrow();
//   }

//   pub fn borrow_mut(&mut self) -> std::cell::RefMut<T> {
//     return self.value.borrow_mut();
//   }
// }

// impl<T> std::fmt::Display for Shareable<T> where T: std::fmt::Display {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     self.value.fmt(f)
//   }
// }

// impl<T> std::cmp::PartialEq for Shareable<T> where T: std::cmp::PartialEq {
//   fn eq(&self, other: &Shareable<T>) -> bool {
//     self.value.eq(&other.value)
//   }
// }
// impl<T> std::cmp::Eq for Shareable<T> where T: std::cmp::Eq {}

// impl<T> std::cmp::PartialOrd for Shareable<T> where T: std::cmp::PartialOrd {
//   fn partial_cmp(&self, other: &Shareable<T>) -> Option<std::cmp::Ordering> {
//     self.value.partial_cmp(&other.value)
//   }
// }
// impl<T> std::cmp::Ord for Shareable<T> where T: std::cmp::Ord {
//   fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//     self.value.cmp(&other.value)
//   }
// }

type Mut<T> = std::cell::RefCell<T>;
type MutRc<T> = std::rc::Rc<Mut<T>>;

pub struct Shareable<T> {
  value: MutRc<T>,
}

impl<T> Shareable<T> {
  pub fn new(value: T) -> Self {
    return Self {
      value: MutRc::new(Mut::new(value)),
    };
  }

  pub fn share(&self) -> Self {
    return Self {
      value: MutRc::clone(&self.value),
    };
  }

  pub fn borrow(&self) -> std::cell::Ref<T> {
    return self.value.borrow();
  }

  pub fn borrow_mut(&mut self) -> std::cell::RefMut<T> {
    return self.value.borrow_mut();
  }

  pub fn try_unique(self) -> Result<T, Self> {
    let res = std::rc::Rc::try_unwrap(self.value);

    return match res {
      Err(rc_value) => Err(Self { value: rc_value }),
      Ok(refcell_value) => Ok(refcell_value.into_inner()),
    }
  }
}

impl<T> std::ops::Deref for Shareable<T> {
  type Target = Mut<T>;

  fn deref(&self) -> &Self::Target {
    return self.value.as_ref();
  }
}

impl<T> std::fmt::Display for Shareable<T> where T: std::fmt::Display {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.value.borrow().fmt(f)
  }
}

impl<T> std::cmp::PartialEq for Shareable<T> where T: std::cmp::PartialEq {
  fn eq(&self, other: &Shareable<T>) -> bool {
    self.value.borrow().eq(&other.value.borrow())
  }
}
impl<T> std::cmp::Eq for Shareable<T> where T: std::cmp::Eq {}

impl<T> std::cmp::PartialOrd for Shareable<T> where T: std::cmp::PartialOrd {
  fn partial_cmp(&self, other: &Shareable<T>) -> Option<std::cmp::Ordering> {
    self.value.borrow().partial_cmp(&other.value.borrow())
  }
}
impl<T> std::cmp::Ord for Shareable<T> where T: std::cmp::Ord {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.value.borrow().cmp(&other.value.borrow())
  }
}
