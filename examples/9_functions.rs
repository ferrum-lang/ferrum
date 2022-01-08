mod lang_prelude;
mod lang_std;

use lang_prelude::*;
use lang_std::{ Console, };

#[allow(non_upper_case_globals)]
const STR_SLICE_0: LangString = LangString::from_slice("Adam");

#[allow(non_upper_case_globals)]
const STR_SLICE_1: LangString = LangString::from_slice(" ");

#[allow(non_upper_case_globals)]
const STR_SLICE_2: LangString = LangString::from_slice("Bates");

#[allow(non_upper_case_globals)]
const STR_SLICE_3: LangString = LangString::from_slice("!");

#[allow(non_upper_case_globals)]
const STR_SLICE_4: LangString = LangString::from_slice("Hello");

#[allow(non_upper_case_globals)]
const STR_SLICE_5: LangString = LangString::from_slice("World");

#[allow(non_upper_case_globals)]
const STR_SLICE_6: LangString = LangString::from_slice("no_args_no_return");

#[allow(non_upper_case_globals)]
const STR_SLICE_7: LangString = LangString::from_slice("no_args_with_return");

fn main() {
  no_args_no_return();
  
  let x = no_args_with_return();
  Console::write_line(LangString::from_owned(format!("Got {}", x)));

  one_arg_no_return(&STR_SLICE_0);

  let x = one_arg_with_return(&STR_SLICE_0);
  Console::write_line(LangString::from_owned(format!("Got: {}", x)));

  two_args_no_return(&STR_SLICE_0, 25);

  let x = two_args_with_return(&STR_SLICE_0, 25);
  Console::write_line(LangString::from_owned(format!("Got: {}", x)));

  var_args(vec![
    STR_SLICE_0,
    STR_SLICE_1,
    STR_SLICE_2,
    STR_SLICE_3,
  ]);

  let x = vec![
    STR_SLICE_4,
    STR_SLICE_1,
    STR_SLICE_5,
  ];
  var_args({
    let mut tmp = vec![];

    let mut x = x;
    tmp.append(&mut x);
    
    tmp
  });
}

fn no_args_no_return() {
  Console::write_line(STR_SLICE_6);
}

fn no_args_with_return() -> LangString {
  Console::write_line(STR_SLICE_7);

  return STR_SLICE_2;
}

fn one_arg_no_return(arg1: &LangString) {
  Console::write_line(LangString::from_owned(format!("one_arg_no_return: {}", arg1)));
}

fn one_arg_with_return(arg1: &LangString) -> LangString {
  Console::write_line(LangString::from_owned(format!("one_arg_with_owned_return: {}", arg1)));

  return STR_SLICE_2;
}

fn two_args_no_return(arg1: &LangString, arg2: isize) {
  Console::write_line(LangString::from_owned(format!("two_args_no_return: {}, {}", arg1, arg2)));
}

fn two_args_with_return(arg1: &LangString, arg2: isize) -> LangString {
  Console::write_line(LangString::from_owned(format!("two_args_with_return: {}, {}", arg1, arg2)));

  return STR_SLICE_2;
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
