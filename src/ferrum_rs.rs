pub mod prelude {
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

    pub struct FeBox<T: std::fmt::Debug> {
        inner: FeBoxInner<T>,
    }

    enum FeBoxInner<T> {
        Unique(T),
        Shared(Rc<UnsafeCell<T>>),
    }

    impl<T: std::fmt::Debug> std::fmt::Debug for FeBox<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let count = self.count();
            let value = self.get();

            return if count == 1 {
                write!(f, "@unique({:?})", value)
            } else {
                write!(f, "@shared({:?})", value)
            };
        }
    }

    impl<T> std::fmt::Display for FeBox<T>
    where
        T: std::fmt::Display + std::fmt::Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            return write!(f, "{}", self.get());
        }
    }

    impl<T: std::fmt::Debug> FeBox<T> {
        pub fn new(value: T) -> Self {
            return Self {
                inner: FeBoxInner::Unique(value),
            };
        }

        // pub fn as_ref(&self) -> &Self {
        //     return &self;
        // }

        pub fn get(&self) -> &T {
            return match &self.inner {
                FeBoxInner::Unique(value) => value,
                FeBoxInner::Shared(cell) => unsafe { &*(cell.get() as *const T) },
            };
        }

        pub fn get_mut(&mut self) -> &mut T {
            return match &mut self.inner {
                FeBoxInner::Unique(value) => value,
                FeBoxInner::Shared(cell) => unsafe { &mut *(cell.get()) },
            };
        }

        pub fn take(self) -> Result<T, Self> {
            return match self.inner {
                FeBoxInner::Unique(value) => Ok(value),
                FeBoxInner::Shared(cell) => match Rc::strong_count(&cell) {
                    0 => unreachable!("Rc count cannot be 0"),
                    1 => {
                        let ptr: *const UnsafeCell<T> = Rc::into_raw(cell);
                        let cell: &UnsafeCell<T> = unsafe { &*ptr };
                        Ok(unsafe { ptr::read(&*cell.get()) })
                    }
                    _ => Err(Self {
                        inner: FeBoxInner::Shared(cell),
                    }),
                },
            };
        }

        pub fn drop(self) {
            return match self.inner {
                FeBoxInner::Unique(_) => {}
                FeBoxInner::Shared(cell) => match Rc::strong_count(&cell) {
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
                FeBoxInner::Unique(_) => 1,
                FeBoxInner::Shared(cell) => {
                    let count = Rc::strong_count(&cell);

                    if count == 1 {
                        unsafe {
                            let cell: &T = self.get();
                            let val: T = ptr::read(cell);
                            let new: FeBoxInner<T> = FeBoxInner::Unique(val);

                            write_to_immut(&self.inner, new);
                        }
                    }

                    count
                }
            };
        }

        pub fn share(&self) -> Self {
            if let FeBoxInner::Unique(_) = self.inner {
                unsafe {
                    let curr: FeBoxInner<T> = ptr::read(&self.inner);

                    // Must not panic before we get to `ptr::write`
                    let new = match curr {
                        FeBoxInner::Unique(val) => {
                            FeBoxInner::Shared(Rc::new(UnsafeCell::new(val)))
                        }
                        FeBoxInner::Shared(rc) => FeBoxInner::Shared(rc),
                    };

                    write_to_immut(&self.inner, new);
                }
            }

            let rc = match &self.inner {
                FeBoxInner::Shared(rc) => rc,
                _ => unreachable!(),
            };

            return Self {
                inner: FeBoxInner::Shared(Rc::clone(&rc)),
            };
        }
    }

    impl<T: std::fmt::Debug> Deref for FeBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            return self.get();
        }
    }

    impl<T: std::fmt::Debug> DerefMut for FeBox<T> {
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

    unsafe fn write_to_immut<T>(immut: &T, value: T) {
        let const_ptr = immut as *const T;
        let mut_ptr = const_ptr as *mut T;
        ptr::write(&mut *mut_ptr, value);
    }
}

pub mod std {}
