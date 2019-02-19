#![feature(try_trait)]
use exit::ExitWithMessage as Exit;

use std::env;
use std::option;

#[derive(Debug)]
enum MyErr {
    MissingArg,
    ParseErrorUserNum,
    ParseErrorGroupNum,
}

impl From<MyErr> for (i32, String) {
    fn from(err: MyErr) -> Self {
        match err {
            MyErr::MissingArg => (2, String::from("An argument is missing")),
            MyErr::ParseErrorUserNum => (3, String::from("Error parsing user number")),
            MyErr::ParseErrorGroupNum => (4, String::from("Error parsing group number")),
        }
    }
}

impl From<option::NoneError> for MyErr {
    fn from(_: option::NoneError) -> Self {
        MyErr::MissingArg
    }
}

// The aliased import allows using ExitWithMessage as Exit
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
