#![feature(try_trait)]
use exit::Exit;

use std::env;
use core::option;
use core::num;

enum MyErr {
    MissingArg,
    ParseIntError,
}

impl From<MyErr> for i32 {
    fn from(err: MyErr) -> Self {
        match err {
            MyErr::MissingArg => 2,
            MyErr::ParseIntError => 3,
        }
    }
}

impl From<option::NoneError> for MyErr {
    fn from(_: option::NoneError) -> Self {
        MyErr::MissingArg
    }
}

impl From<num::ParseIntError> for MyErr {
    fn from(_: num::ParseIntError) -> Self {
        MyErr::ParseIntError
    }
}

fn main() -> Exit<MyErr> {
    let num_string= env::args().skip(1).next()?;
    let num : u32 = num_string.parse()?;

    println!("Hello, user #{}!", num);

    Exit::Ok
}
