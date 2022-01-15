use lang_prelude::*;

fn main() {}

#[allow(dead_code)]
pub type Void = ();

#[allow(dead_code)]
pub struct Console {}
impl Console {
    #[allow(dead_code)]
    pub fn write_line(text: LangString) {
        println!("{}", text);
    }
}

#[allow(dead_code)]
pub struct LangStringBuilder {
    state: String,
}
impl LangStringBuilder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            state: String::new(),
        }
    }

    #[allow(dead_code)]
    pub fn from(string: LangString) -> Self {
        Self {
            state: string.as_owned(),
        }
    }

    #[allow(dead_code)]
    pub fn with_prepend(mut self, string: LangString) -> Self {
        self.state.insert_str(0, string.as_slice());
        return self;
    }

    #[allow(dead_code)]
    pub fn with_append(mut self, string: LangString) -> Self {
        self.state.push_str(string.as_slice());
        return self;
    }

    #[allow(dead_code)]
    pub fn prepend(&mut self, string: LangString) {
        self.state.insert_str(0, string.as_slice());
    }

    #[allow(dead_code)]
    pub fn append(&mut self, string: LangString) {
        self.state.push_str(string.as_slice());
    }

    #[allow(dead_code)]
    pub fn build(self) -> LangString {
        LangString::from_owned(self.state)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Error {
    message: Option<LangString>,
}
impl Error {
    #[allow(dead_code)]
    pub fn new(message: LangString) -> Self {
        Self {
            message: Some(message),
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

pub struct UUID {
    value: String,
}
impl UUID {
    pub fn from_seed(seed: &LangString) -> Self {
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
