#![feature(try_trait)]
use exit::{Exit, ExitDisplay};

use std::env;
use std::option;

#[derive(Debug)]
enum MyErr {
    MissingArg,
    ParseErrorUserNum,
    ParseErrorGroupNum,
}

impl From<MyErr> for i32 {
    fn from(err: MyErr) -> Self {
        match err {
            MyErr::MissingArg => 2,
            MyErr::ParseErrorUserNum => 3,
            MyErr::ParseErrorGroupNum => 4,
        }
    }
}

// You can optionally implement ExitDisplay for your error type in order to print an error message
// on exit
impl ExitDisplay for MyErr {
    fn display(&self) -> String {
        format!("{:?}", self)
    }
}

impl From<option::NoneError> for MyErr {
    fn from(_: option::NoneError) -> Self {
        MyErr::MissingArg
    }
}

fn main() -> Exit<MyErr> {
    let user_num_string : String = env::args().skip(1).next()?;
    let group_num_string : String = env::args().skip(2).next()?;

    let user_num : u32 = user_num_string.parse()
        .map_err(|_| MyErr::ParseErrorUserNum)?;
    let group_num : u32 = group_num_string.parse()
        .map_err(|_| MyErr::ParseErrorGroupNum)?;

    println!("Hello, user #{} from group #{}!", user_num, group_num);

    Exit::Ok
}
