mod lang_std;

use lang_std::{ Console };

fn main() {
  let x: isize = 2;
  let y: isize = 3;

  Console::write_line(x + y);
  Console::write_line(x - y); 
  Console::write_line(x * y);
  Console::write_line(x as f64 / y as f64);
  Console::write_line(x.pow(y as u32));
}