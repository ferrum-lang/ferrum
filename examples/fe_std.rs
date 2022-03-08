use fe_prelude::*;

fn main() {}

#[allow(dead_code)]
pub type Void = ();

#[allow(dead_code)]
pub struct Console {}
impl Console {
    #[allow(dead_code)]
    pub fn write_line<S: Into<FeString>>(text: S) {
        println!("{}", text.into());
    }
}

#[allow(dead_code)]
pub struct FeStringBuilder {
    state: String,
}
impl FeStringBuilder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            state: String::new(),
        }
    }

    #[allow(dead_code)]
    pub fn from(string: FeString) -> Self {
        Self {
            state: string.as_owned(),
        }
    }

    #[allow(dead_code)]
    pub fn with_prepend(mut self, string: FeString) -> Self {
        self.state.insert_str(0, string.as_slice());
        return self;
    }

    #[allow(dead_code)]
    pub fn with_append(mut self, string: FeString) -> Self {
        self.state.push_str(string.as_slice());
        return self;
    }

    #[allow(dead_code)]
    pub fn prepend(&mut self, string: FeString) {
        self.state.insert_str(0, string.as_slice());
    }

    #[allow(dead_code)]
    pub fn append(&mut self, string: FeString) {
        self.state.push_str(string.as_slice());
    }

    #[allow(dead_code)]
    pub fn build(self) -> FeString {
        FeString::from_owned(self.state)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Error {
    message: Option<FeString>,
}
impl Error {
    #[allow(dead_code)]
    pub fn new<T: Into<FeString>>(message: T) -> Self {
        Self {
            message: Some(message.into()),
        }
    }

    #[allow(dead_code)]
    pub fn empty() -> Self {
        Self { message: None }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}
impl std::error::Error for Error {}

#[allow(dead_code)]
pub type Map<K, V> = std::collections::HashMap<K, V>;

#[allow(dead_code)]
pub type Set<K> = std::collections::HashSet<K>;

#[derive(Debug)]
pub struct UUID {
    value: String,
}
impl UUID {
    pub fn from_seed(seed: &FeString) -> Self {
        let memory_address = seed.clone().as_owned().as_ptr() as usize;
        return Self {
            value: format!("{}", memory_address),
        };
    }
}
impl std::clone::Clone for UUID {
    fn clone(&self) -> Self {
        let memory_address = self.value.as_ptr() as usize;
        return Self {
            value: format!("{}", memory_address),
        };
    }
}
impl std::fmt::Display for UUID {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.value)
    }
}
