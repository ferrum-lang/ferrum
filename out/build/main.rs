mod lang_prelude;
use lang_prelude::*;
mod lang_std;
use lang_std::{
Console,
};
fn math(
)
{
Console::write_line(LangString::from_owned(format!("1 + 2 = {}", 3)));
}
fn main(
)
{
Console::write_line(LangString::from_slice("Hello world!"));
math();
}
