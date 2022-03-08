#![feature(const_fn_trait_bound)]

mod fe_prelude;
mod fe_std;

use fe_prelude::*;

#[allow(non_upper_case_globals)]
const STR_SLICE_0: FeString = FeString::from_slice("Adam");

#[allow(non_upper_case_globals)]
const STR_SLICE_1: FeString = FeString::from_slice("abc");

#[allow(non_upper_case_globals)]
const STR_SLICE_2: FeString = FeString::from_slice("uh oh!");

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
    let x: BigUint = BigUint::new(0);

    let x: u8 = false as u8;
    let x: u8 = 0;

    let x: i8 = 0;
    let x: i16 = 0;
    let x: i32 = 0;
    let x: i64 = 0;
    let x: i128 = 0;
    let x: isize = 0;
    let x: BigInt = BigInt::new(0);

    let x: isize = 0;

    let x: f32 = 0 as f32;
    let x: f64 = 0 as f64;

    let x: char = 'a';

    let x: FeString = STR_SLICE_0;
    let x: FeString = FeString::from_owned(format!("Adam {}", x));

    let x: (usize, usize, usize) = (1, 2, 3);
    let x: (isize, FeString, char) = (42, STR_SLICE_0, 'F');

    let y: isize = x.0;
    let y: FeString = x.1;
    let y: char = x.2;

    let x: Vec<isize> = vec![1, 2, 3];
    let x: Vec<isize> = {
        let mut x = Vec::new();
        for i in 0..3 {
            x.push(i + 1);
        }
        x
    };

    let x: Option<isize> = None;
    let x: Option<isize> = Some(123);
    let x: Option<isize> = Some(123);

    let x: bool = (None as Option<()>).is_some();
    let x: bool = (None as Option<()>).is_none();

    let x: isize = Some(123).unwrap();
    let x: isize = None.unwrap_or(0);
    let x: isize = None.unwrap_or_else(|| 0);

    let x: isize = None.unwrap_or(0);
    let x: isize = None.unwrap_or_else(|| 0);

    let x: Option<FeString> = Some(STR_SLICE_1);
    let y: Option<usize> = x.map(|x| x.length);

    let x: Option<FeString> = Some(STR_SLICE_1);
    let y: usize = x.map(|x| x.length).unwrap_or(0);

    let x: Result<isize, fe_std::Error> = None.ok_or(fe_std::Error::new(STR_SLICE_2));
    let x: Result<isize, FeString> = None.ok_or(STR_SLICE_2);
    let x: Result<isize, FeString> = {
        let x: Option<isize> = None;
        if let Some(x) = x {
            Ok(x)
        } else {
            let f = || STR_SLICE_2;
            Err(f())
        }
    };
    let x = {
        let x: Option<isize> = None;
        if let Some(x) = x {
            Ok(x)
        } else {
            let f = || STR_SLICE_2;
            Err(f())
        }
    };

    let x: Result<isize, fe_std::Error> = None.ok_or(fe_std::Error::empty());

    let x: Result<(), fe_std::Error> = Ok(());
    let x: Result<(), fe_std::Error> = Ok(());
    let x: Result<(), fe_std::Error> = Ok(());
    let x: Result<(), fe_std::Error> = Err(fe_std::Error::empty());

    let x: Result<isize, FeString> = Err(STR_SLICE_2);
    let x: Result<isize, FeString> = Ok(123);

    let x: bool = {
        let x: Result<isize, fe_std::Error> = Ok(123);
        x.is_ok()
    };
    let x: bool = {
        let x: Result<isize, fe_std::Error> = Ok(123);
        x.is_err()
    };

    let x: Option<isize> = {
        let x: Result<isize, fe_std::Error> = Ok(123);
        x.ok()
    };
    let x: Option<fe_std::Error> = {
        let x: Result<isize, fe_std::Error> = Ok(123);
        x.err()
    };
}
