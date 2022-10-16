use crate::ferrum::prelude::*;

pub fn say_hello() {
    inner();
}
fn inner() {
    print(FeStr::from_static("Hello world!"));
}
