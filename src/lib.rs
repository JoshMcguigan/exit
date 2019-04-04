#![feature(termination_trait_lib)]
#![feature(try_trait)]
extern crate failure;

use failure::Fail;

use std::process::Termination;
use std::ops::Try;

pub enum Exit<T> {
    Ok,
    Err(T)
}

impl<T: Into<i32> + Fail> Termination for Exit<T> {
    fn report(self) -> i32 {
        match self {
            Exit::Ok => 0,
            Exit::Err(err) => {
                eprintln!("Error: {}", err);
                err.into()
            },
        }
    }
}

impl<T> Try for Exit<T> {
    type Ok = ();
    type Error = T;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Exit::Ok => Ok(()),
            Exit::Err(err) => Err(err)
        }
    }

    fn from_error(err: Self::Error) -> Self {
        Exit::Err(err)
    }

    fn from_ok(_: Self::Ok) -> Self {
        Exit::Ok
    }
}
