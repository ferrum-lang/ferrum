use std::{clone, cmp, fmt, hash, ops, rc};

fn main() {}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum FeStringValue {
    Slice(&'static str),
    Owned(String),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FeString {
    value: FeStringValue,
    pub length: usize,
}
impl FeString {
    #[allow(dead_code)]
    pub const fn from_slice(slice: &'static str) -> Self {
        Self {
            length: slice.len(),
            value: FeStringValue::Slice(slice),
        }
    }

    #[allow(dead_code)]
    pub fn from_owned(string: String) -> Self {
        Self {
            length: string.len(),
            value: FeStringValue::Owned(string),
        }
    }

    #[allow(dead_code)]
    pub fn as_slice(&self) -> &str {
        match &self.value {
            FeStringValue::Slice(x) => x,
            FeStringValue::Owned(x) => &*x,
        }
    }

    #[allow(dead_code)]
    pub fn as_owned(self) -> String {
        match self.value {
            FeStringValue::Slice(x) => x.to_string(),
            FeStringValue::Owned(x) => x,
        }
    }
}
impl From<bool> for FeString {
    fn from(value: bool) -> Self {
        FeString::from_owned(value.to_string())
    }
}
impl From<u8> for FeString {
    fn from(value: u8) -> Self {
        FeString::from_owned(value.to_string())
    }
}
impl From<u16> for FeString {
    fn from(value: u16) -> Self {
        FeString::from_owned(value.to_string())
    }
}
impl From<u32> for FeString {
    fn from(value: u32) -> Self {
        FeString::from_owned(value.to_string())
    }
}
impl From<u64> for FeString {
    fn from(value: u64) -> Self {
        FeString::from_owned(value.to_string())
    }
}
impl From<u128> for FeString {
    fn from(value: u128) -> Self {
        FeString::from_owned(value.to_string())
    }
}
impl From<usize> for FeString {
    fn from(value: usize) -> Self {
        FeString::from_owned(value.to_string())
    }
}
impl From<i8> for FeString {
    fn from(value: i8) -> Self {
        FeString::from_owned(value.to_string())
    }
}
impl From<i16> for FeString {
    fn from(value: i16) -> Self {
        FeString::from_owned(value.to_string())
    }
}
impl From<i32> for FeString {
    fn from(value: i32) -> Self {
        FeString::from_owned(value.to_string())
    }
}
impl From<i64> for FeString {
    fn from(value: i64) -> Self {
        FeString::from_owned(value.to_string())
    }
}
impl From<i128> for FeString {
    fn from(value: i128) -> Self {
        FeString::from_owned(value.to_string())
    }
}
impl From<isize> for FeString {
    fn from(value: isize) -> Self {
        FeString::from_owned(value.to_string())
    }
}
impl From<char> for FeString {
    fn from(value: char) -> Self {
        FeString::from_owned(value.to_string())
    }
}
impl<T> From<FeShareable<T>> for FeString
where
    T: fmt::Debug + clone::Clone + Into<FeString>,
{
    fn from(value: FeShareable<T>) -> Self {
        let value = match value.try_mutable() {
            Ok(value) => value,
            Err(value) => value.clone().take(),
        };

        return value.into();
    }
}
impl PartialEq for FeString {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}
impl Eq for FeString {}
impl hash::Hash for FeString {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}
impl fmt::Display for FeString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match &self.value {
            FeStringValue::Slice(x) => fmt::Display::fmt(x, f),
            FeStringValue::Owned(x) => fmt::Display::fmt(x, f),
        };
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

#[derive(Debug)]
enum FeShareableValue<T> {
    Unique(T),
    Shared(rc::Rc<T>),
}

#[derive(Debug)]
pub struct FeShareable<T: fmt::Debug> {
    value: FeShareableValue<T>,
}
impl<T> FeShareable<T>
where
    T: fmt::Debug,
{
    pub const fn new(value: T) -> Self {
        return Self {
            value: FeShareableValue::Unique(value),
        };
    }

    pub fn share(self) -> (Self, Self) {
        match self.value {
            FeShareableValue::Shared(value) => {
                let shared = Self {
                    value: FeShareableValue::Shared(rc::Rc::clone(&value)),
                };

                return (
                    Self {
                        value: FeShareableValue::Shared(value),
                    },
                    shared,
                );
            }
            FeShareableValue::Unique(value) => {
                let value = rc::Rc::new(value);

                let shared = Self {
                    value: FeShareableValue::Shared(rc::Rc::clone(&value)),
                };

                return (
                    Self {
                        value: FeShareableValue::Shared(value),
                    },
                    shared,
                );
            }
        };
    }

    pub fn try_mutable(self) -> Result<T, Self> {
        return match self.value {
            FeShareableValue::Unique(value) => Ok(value),
            FeShareableValue::Shared(value) => match rc::Rc::try_unwrap(value) {
                Ok(value) => Ok(value),
                Err(value) => Err(Self {
                    value: FeShareableValue::Shared(value),
                }),
            },
        };
    }

    pub fn take(self) -> T {
        return match self.try_mutable() {
            Ok(value) => value,
            Err(this) => panic!(
                "Cannot take ownership of data while multiple shared references exist: {:?}",
                this
            ),
        };
    }
}
impl<T> ops::Deref for FeShareable<T>
where
    T: fmt::Debug,
{
    type Target = T;

    fn deref(&self) -> &T {
        return match &self.value {
            FeShareableValue::Unique(value) => value,
            FeShareableValue::Shared(value) => value,
        };
    }
}
impl<T> clone::Clone for FeShareable<T>
where
    T: fmt::Debug + clone::Clone,
{
    fn clone(&self) -> Self {
        let value: &T = &self;
        return Self::new(value.clone());
    }
}
impl<T> fmt::Display for FeShareable<T>
where
    T: fmt::Debug + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value: &T = &self;
        return fmt::Display::fmt(value, f);
    }
}
impl<T> cmp::PartialEq for FeShareable<T>
where
    T: fmt::Debug + cmp::PartialEq,
{
    fn eq(&self, other: &FeShareable<T>) -> bool {
        let value: &T = &self;
        return cmp::PartialEq::eq(value, &other);
    }
}
impl<T> cmp::Eq for FeShareable<T> where T: fmt::Debug + cmp::Eq {}
impl<T> cmp::PartialOrd for FeShareable<T>
where
    T: fmt::Debug + cmp::PartialOrd,
{
    fn partial_cmp(&self, other: &FeShareable<T>) -> Option<cmp::Ordering> {
        let value: &T = &self;
        return cmp::PartialOrd::partial_cmp(value, &other);
    }
}
impl<T> cmp::Ord for FeShareable<T>
where
    T: fmt::Debug + cmp::Ord,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let value: &T = &self;
        return cmp::Ord::cmp(value, &other);
    }
}
impl<T> hash::Hash for FeShareable<T>
where
    T: fmt::Debug + hash::Hash,
{
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        let value: &T = &self;
        return value.hash(state);
    }
}

pub trait ShareSub {
    fn on_share(&self);
}
impl<T> ShareSub for FeShareable<T>
where
    T: fmt::Debug + ShareSub,
{
    fn on_share(&self) {
        let value: &T = &self;
        value.on_share();
    }
}

#[derive(Debug)]
enum FeMutFieldValue<T: fmt::Debug> {
    Mut(T),
    Immut(FeShareable<T>),
}

#[derive(Debug)]
pub struct FeMutField<T: fmt::Debug> {
    value: FeMutFieldValue<T>,
}
impl<T: fmt::Debug> FeMutField<T> {
    pub fn new(value: T) -> Self {
        return Self {
            value: FeMutFieldValue::Mut(value),
        };
    }

    pub fn try_mutable(self) -> Result<T, Self> {
        return match self.value {
            FeMutFieldValue::Mut(value) => Ok(value),
            FeMutFieldValue::Immut(value) => match value.try_mutable() {
                Ok(value) => Ok(value),
                Err(this) => Err(Self::from(this)),
            },
        };
    }

    pub fn take(self) -> T {
        return match self.value {
            FeMutFieldValue::Mut(value) => value,
            FeMutFieldValue::Immut(value) => value.take(),
        };
    }
}
impl<T: fmt::Debug> ops::Deref for FeMutField<T> {
    type Target = T;

    fn deref(&self) -> &T {
        return match &self.value {
            FeMutFieldValue::Mut(value) => value,
            FeMutFieldValue::Immut(value) => value,
        };
    }
}
impl<T: fmt::Debug> ops::DerefMut for FeMutField<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return match &mut self.value {
            FeMutFieldValue::Mut(value) => value,
            _ => panic!("Cannot mutably borrow immutable data!"),
        };
    }
}
impl<T> clone::Clone for FeMutField<T>
where
    T: fmt::Debug + clone::Clone,
{
    fn clone(&self) -> Self {
        let value: &T = &self;
        return Self::new(value.clone());
    }
}
impl<T: fmt::Debug> From<FeShareable<T>> for FeMutField<T> {
    fn from(shareable: FeShareable<T>) -> Self {
        return Self {
            value: FeMutFieldValue::Immut(shareable),
        };
    }
}
