mod lang_std;

fn main() {
  let x: bool = true;
  let x: bool = false;

  let x: u8 = false as u8;
  let x: u8 = 0;
  let x: u16 = 0;
  let x: u32 = 0;
  let x: u64 = 0;
  let x: u128 = 0;
  let x: usize = 0;
  let x: lang_std::BigUint = lang_std::BigUint::new(0);

  let x: u8 = false as u8;
  let x: u8 = 0;

  let x: i8 = 0;
  let x: i16 = 0;
  let x: i32 = 0;
  let x: i64 = 0;
  let x: i128 = 0;
  let x: isize = 0;
  let x: lang_std::BigInt = lang_std::BigInt::new(0);

  let x: isize = 0;

  let x: f32 = 0 as f32;
  let x: f64 = 0 as f64;

  let x: char = 'a';

  let x: &str = "Adam";
  let x: &str = &*format!("Adam {}", x);

  let x: (usize, usize, usize) = (1, 2, 3);
  let x: (isize, &str, char) = (42, "Adam", 'F');

  let y: isize = x.0;
  let y: &str = x.1;
  let y: char = x.2;

  let x: Vec<isize> = vec![1, 2, 3];
  let x: Vec<isize> = {
    let mut x = Vec::new();
    for i in 0..3 { x.push(i + 1); }
    x
  };

  let x: Option<isize> = None;
  let x: Option<isize> = Some(123);

  let x: bool = (None as Option<()>).is_some();
  let x: bool = (None as Option<()>).is_none();

  let x: isize = Some(123).unwrap();
  let x: isize = None.unwrap_or(0);
  let x: isize = None.unwrap_or_else(|| 0);

  let x: Result<isize, lang_std::Error> = None.ok_or(lang_std::Error::new("uh oh!"));
  let x: Result<isize, &str> = None.ok_or("uh oh!");
  let x: Result<isize, &str> = {
    let x: Option<isize> = None;
    if let Some(x) = x {
      Ok(x)
    } else {
      let f = || "uh oh";
      Err(f())
    }
  };

  let x: Result<(), lang_std::Error> = Ok(());
  let x: Result<(), lang_std::Error> = Ok(());
  let x: Result<(), lang_std::Error> = Ok(());
  let x: Result<(), lang_std::Error> = Err(lang_std::Error::empty());

  let x: Result<isize, &str> = Err("uh oh!");
  let x: Result<isize, &str> = Ok(123);

  let x: bool = {
    let x: Result<isize, lang_std::Error> = Ok(123);
    x.is_ok()
  };
  let x: bool = {
    let x: Result<isize, lang_std::Error> = Ok(123);
    x.is_err()
  };

  let x: Option<isize> = {
    let x: Result<isize, lang_std::Error> = Ok(123);
    x.ok()
  };
  let x: Option<lang_std::Error> = {
    let x: Result<isize, lang_std::Error> = Ok(123);
    x.err()
  };
}