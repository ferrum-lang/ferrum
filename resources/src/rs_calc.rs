use crate::ferrum::prelude::*;

pub fn get_longest<'a>(a: &'a FeStr, b: &'a FeStr) -> &'a FeStr {
    if a.len() >= b.len() {
        return a;
    } else {
        return b;
    }
}
