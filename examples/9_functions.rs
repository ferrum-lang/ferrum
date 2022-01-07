mod lang_prelude;
mod lang_std;

use lang_prelude::*;
use lang_std::{ Console, LangStringBuilder, };

fn main() {
  no_args_no_return();
  
  let x = no_args_with_return();
  Console::write_line(LangString::from_owned(format!("Got {}", x)));

  one_arg_no_return(&LangString::from_slice("Adam"));

  let x = one_arg_with_return(&LangString::from_slice("Adam"));
  Console::write_line(LangString::from_owned(format!("Got: {}", x)));

  two_args_no_return(&LangString::from_slice("Adam"), 25);

  let x = two_args_with_return(&LangString::from_slice("Adam"), 25);
  Console::write_line(LangString::from_owned(format!("Got: {}", x)));

  var_args(vec![
    LangString::from_slice("Adam"),
    LangString::from_slice(" "),
    LangString::from_slice("Bates"),
    LangString::from_slice("!"),
  ]);

  let x = vec![
    LangString::from_slice("Hello"),
    LangString::from_slice(" "),
    LangString::from_slice("World"),
  ];
  var_args({
    let mut tmp = vec![];

    let mut x = x;
    tmp.append(&mut x);
    
    tmp
  });
}

fn no_args_no_return() {
  Console::write_line(LangString::from_slice("no_args_no_return"));
}

fn no_args_with_return() -> LangString {
  Console::write_line(LangString::from_slice("no_args_with_return"));

  return LangString::from_slice("Bates");
}

fn one_arg_no_return(arg1: &LangString) {
  Console::write_line(LangString::from_owned(format!("one_arg_no_return: {}", arg1)));
}

fn one_arg_with_return(arg1: &LangString) -> LangString {
  Console::write_line(LangString::from_owned(format!("one_arg_with_owned_return: {}", arg1)));

  return LangString::from_slice("Bates");
}

fn two_args_no_return(arg1: &LangString, arg2: isize) {
  Console::write_line(LangString::from_owned(format!("two_args_no_return: {}, {}", arg1, arg2)));
}

fn two_args_with_return(arg1: &LangString, arg2: isize) -> LangString {
  Console::write_line(LangString::from_owned(format!("two_args_with_return: {}, {}", arg1, arg2)));

  return LangString::from_slice("Bates");
}

fn var_args(args: Vec<LangString>) {
  Console::write_line(LangString::from_owned(format!("var_args: {}",
    args
      .iter()
      .map(|e| e.to_string())
      .collect::<Vec<String>>()
      .join(", ")
  )));
}
