pub mod std {
    pub struct Map {}
}
pub mod prelude {
    use super::utils;
    use std::cell::UnsafeCell;
    use std::marker::PhantomData;
    use std::ops::{Deref, DerefMut};
    use std::ptr;
    use std::rc::Rc;
    pub fn print(string: impl std::fmt::Display) {
        println!("{string}");
    }
    pub fn phantom() -> PhantomData<()> {
        return PhantomData;
    }
    pub struct FeShared<T: std::fmt::Debug> {
        inner: FeSharedInner<T>,
    }
    enum FeSharedInner<T> {
        Unique(T),
        Shared(Rc<UnsafeCell<T>>),
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for FeShared<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let count = self.count();
            let value = self.get();
            return if count == 1 {
                write!(f, "@unique({:#?})", value)
            } else {
                write!(f, "@shared({:#?})", value)
            };
        }
    }
    impl<T> std::fmt::Display for FeShared<T>
    where
        T: std::fmt::Display + std::fmt::Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            return write!(f, "{}", self.get());
        }
    }
    impl<T> std::clone::Clone for FeShared<T>
    where
        T: std::clone::Clone + std::fmt::Debug,
    {
        fn clone(&self) -> Self {
            return FeShared::new(self.get().clone());
        }
    }
    impl<T: std::fmt::Debug> FeShared<T> {
        pub fn new(value: T) -> Self {
            return Self {
                inner: FeSharedInner::Unique(value),
            };
        }
        pub fn get(&self) -> &T {
            return match &self.inner {
                FeSharedInner::Unique(value) => value,
                FeSharedInner::Shared(cell) => unsafe { &*(cell.get() as *const T) },
            };
        }
        pub fn get_mut(&mut self) -> &mut T {
            return match &mut self.inner {
                FeSharedInner::Unique(value) => value,
                FeSharedInner::Shared(cell) => unsafe { &mut *(cell.get()) },
            };
        }
        pub fn take(self) -> Result<T, Self> {
            return match self.inner {
                FeSharedInner::Unique(value) => Ok(value),
                FeSharedInner::Shared(cell) => match Rc::strong_count(&cell) {
                    0 => unreachable!("Rc count cannot be 0"),
                    1 => {
                        let ptr: *const UnsafeCell<T> = Rc::into_raw(cell);
                        let cell: &UnsafeCell<T> = unsafe { &*ptr };
                        Ok(unsafe { ptr::read(&*cell.get()) })
                    }
                    _ => Err(Self {
                        inner: FeSharedInner::Shared(cell),
                    }),
                },
            };
        }
        pub fn drop(self) {
            return match self.inner {
                FeSharedInner::Unique(_) => {}
                FeSharedInner::Shared(cell) => match Rc::strong_count(&cell) {
                    0 => unreachable!("Rc count cannot be 0"),
                    1 => {
                        let ptr: *const UnsafeCell<T> = Rc::into_raw(cell);
                        let cell: &UnsafeCell<T> = unsafe { &*ptr };
                        let _ = unsafe { ptr::read(&*cell.get()) };
                    }
                    _ => drop(cell),
                },
            };
        }
        pub fn count(&self) -> usize {
            return match &self.inner {
                FeSharedInner::Unique(_) => 1,
                FeSharedInner::Shared(cell) => {
                    let count = Rc::strong_count(&cell);
                    if count == 1 {
                        unsafe {
                            let cell: &T = self.get();
                            let val: T = ptr::read(cell);
                            let new: FeSharedInner<T> = FeSharedInner::Unique(val);
                            utils::mem::write_to_immut(&self.inner, new);
                        }
                    }
                    count
                }
            };
        }
        pub fn share(&self) -> Self {
            if let FeSharedInner::Unique(_) = self.inner {
                unsafe {
                    let curr: FeSharedInner<T> = ptr::read(&self.inner);
                    let new = match curr {
                        FeSharedInner::Unique(val) => {
                            FeSharedInner::Shared(Rc::new(UnsafeCell::new(val)))
                        }
                        FeSharedInner::Shared(rc) => FeSharedInner::Shared(rc),
                    };
                    utils::mem::write_to_immut(&self.inner, new);
                }
            }
            let rc = match &self.inner {
                FeSharedInner::Shared(rc) => rc,
                _ => unreachable!(),
            };
            return Self {
                inner: FeSharedInner::Shared(Rc::clone(&rc)),
            };
        }
    }
    impl<T: std::fmt::Debug> Deref for FeShared<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            return self.get();
        }
    }
    impl<T: std::fmt::Debug> DerefMut for FeShared<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            return self.get_mut();
        }
    }
    pub struct FeStr {
        inner: FeStrInner,
    }
    enum FeStrInner {
        StaticSlice(&'static str),
        Owned(String),
    }
    impl FeStr {
        pub fn from_static(value: &'static str) -> Self {
            return Self {
                inner: FeStrInner::StaticSlice(value),
            };
        }
        pub fn from_owned(value: String) -> Self {
            return Self {
                inner: FeStrInner::Owned(value),
            };
        }
        pub fn append(&mut self, other: FeStr) {
            self.get_owned_mut().push_str(other.deref());
        }
        fn get_owned_mut(&mut self) -> &mut String {
            self.ensure_owned();
            return match &mut self.inner {
                FeStrInner::StaticSlice(_) => unreachable!(),
                FeStrInner::Owned(string) => string,
            };
        }
        fn ensure_owned(&mut self) {
            if let FeStrInner::StaticSlice(slice) = self.inner {
                let string = slice.to_string();
                self.inner = FeStrInner::Owned(string);
            }
        }
    }
    impl From<&'static str> for FeStr {
        fn from(string: &'static str) -> Self {
            return Self::from_static(string);
        }
    }
    impl From<String> for FeStr {
        fn from(string: String) -> Self {
            return Self::from_owned(string);
        }
    }
    impl Deref for FeStr {
        type Target = str;
        fn deref(&self) -> &Self::Target {
            return match &self.inner {
                FeStrInner::StaticSlice(slice) => slice,
                FeStrInner::Owned(string) => string.as_str(),
            };
        }
    }
    impl std::fmt::Display for FeStr {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            return self.deref().fmt(f);
        }
    }
    impl std::fmt::Debug for FeStr {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            return write!(f, "\"{}\"", self);
        }
    }
}
mod utils {
    pub mod mem {
        use std::ptr;
        pub unsafe fn write_to_immut<T>(immut: &T, value: T) {
            let const_ptr = immut as *const T;
            let mut_ptr = const_ptr as *mut T;
            ptr::write(&mut *mut_ptr, value);
        }
    }
}
