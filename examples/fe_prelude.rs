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

impl PartialEq for FeString {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}
impl Eq for FeString {}

impl std::hash::Hash for FeString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

impl std::fmt::Display for FeString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match &self.value {
            FeStringValue::Slice(x) => write!(f, "{}", x),
            FeStringValue::Owned(x) => write!(f, "{}", x),
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

#[allow(dead_code)]
type Mut<T> = std::cell::RefCell<T>;

#[allow(dead_code)]
type MutRc<T> = std::rc::Rc<Mut<T>>;

pub enum Instance<'a, T> {
    Immutable(&'a T),
    Mutable(&'a mut T),
}
impl<'a, T> Instance<'a, T> {
    pub fn try_mutable(self) -> Result<&'a mut T, &'a T> {
        match self {
            Self::Mutable(mutable) => Ok(mutable),
            Self::Immutable(immutable) => Err(immutable),
        }
    }
}

pub trait Share<T> {
    fn on_share(&self) {}
}

#[allow(dead_code)]
pub struct Shareable<T> {
    value: MutRc<T>,
}

impl<T> Shareable<T>
where
    T: Share<T>,
{
    #[allow(dead_code)]
    pub fn new(value: T) -> Self {
        return Self {
            value: MutRc::new(Mut::new(value)),
        };
    }

    #[allow(dead_code)]
    pub fn share(&self) -> Self {
        let shared: Shareable<T> = Self {
            value: MutRc::clone(&self.value),
        };

        let borrow: &T = &self.borrow();
        borrow.on_share();

        return shared;
    }

    #[allow(dead_code)]
    pub fn borrow(&self) -> std::cell::Ref<T> {
        return self.value.borrow();
    }

    #[allow(dead_code)]
    pub fn borrow_mut(&mut self) -> std::cell::RefMut<T> {
        return self.value.borrow_mut();
    }

    #[allow(dead_code)]
    pub fn try_unique(self) -> Result<T, Self> {
        let res = std::rc::Rc::try_unwrap(self.value);

        return match res {
            Err(rc_value) => Err(Self { value: rc_value }),
            Ok(refcell_value) => Ok(refcell_value.into_inner()),
        };
    }
}

impl<T> std::ops::Deref for Shareable<T> {
    type Target = Mut<T>;

    fn deref(&self) -> &Self::Target {
        return self.value.as_ref();
    }
}

impl<T> std::fmt::Display for Shareable<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.borrow().fmt(f)
    }
}

impl<T> std::cmp::PartialEq for Shareable<T>
where
    T: std::cmp::PartialEq,
{
    fn eq(&self, other: &Shareable<T>) -> bool {
        self.value.borrow().eq(&other.value.borrow())
    }
}
impl<T> std::cmp::Eq for Shareable<T> where T: std::cmp::Eq {}

impl<T> std::cmp::PartialOrd for Shareable<T>
where
    T: std::cmp::PartialOrd,
{
    fn partial_cmp(&self, other: &Shareable<T>) -> Option<std::cmp::Ordering> {
        self.value.borrow().partial_cmp(&other.value.borrow())
    }
}
impl<T> std::cmp::Ord for Shareable<T>
where
    T: std::cmp::Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.borrow().cmp(&other.value.borrow())
    }
}

impl<T> std::hash::Hash for Shareable<T>
where
    T: std::hash::Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.borrow().hash(state)
    }
}
