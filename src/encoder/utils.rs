pub fn format_and_join<T: std::fmt::Display>(elems: &Vec<T>, delim: &'static str) -> String {
  elems
    .iter()
    .map(|elem| format!("{}", elem))
    .collect::<Vec<String>>()
    .join(delim)
}
