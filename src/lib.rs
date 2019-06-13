#![feature(termination_trait_lib)]
#![feature(try_trait)]

use std::fmt::Debug;
use std::ops::Try;
use std::process::Termination;

pub enum Exit<T> {
    Ok,
    Err(T),
}

impl<T: Into<i32> + Debug> Termination for Exit<T> {
    fn report(self) -> i32 {
        match self {
            Exit::Ok => 0,
            Exit::Err(err) => {
                eprintln!("Error: {:?}", err);
                err.into()
            }
        }
    }
}

impl<T> Try for Exit<T> {
    type Ok = ();
    type Error = T;

    fn into_result(self) -> Result<<Self as Try>::Ok, Self::Error> {
        match self {
            Exit::Ok => Ok(()),
            Exit::Err(err) => Err(err),
        }
    }

    fn from_error(err: Self::Error) -> Self {
        Exit::Err(err)
    }

    fn from_ok(_: <Self as Try>::Ok) -> Self {
        Exit::Ok
    }
}
