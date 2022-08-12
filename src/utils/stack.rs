use std::fmt;
use std::fmt::Debug;

pub struct Stack<T> {
    values: Vec<T>,
}

impl<T> Stack<T>
    where T: Clone,
{
    pub fn from_top_to_bottom_vec(vec: Vec<T>) -> Self {
        let mut values = vec.clone();
        values.reverse();

        return Self { values };
    }
}

impl<T> Stack<T> {
    pub fn pop(&mut self) -> Option<T> {
        return self.values.pop();
    }

    pub fn push(&mut self, value: T) {
        self.values.push(value);
    }

    pub fn peek(&self) -> Option<&T> {
        return self.values.last();
    }

    pub fn is_empty(&self) -> bool {
        return self.values.is_empty();
    }

    pub fn len(&self) -> usize {
        return self.values.len();
    }
}

impl<T> Debug for Stack<T> where T: Clone + Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut values = self.values.clone();
        values.reverse();

        write!(f, "{values:?}")
    }
}

