#![feature(termination_trait_lib)]
#![feature(try_trait)]

use std::process::Termination;
use std::fmt::Debug;
use std::ops::Try;

pub enum Exit<T> {
    Ok,
    Err(T)
}

impl<T: Into<i32> + Debug> Termination for Exit<T> {
    fn report(self) -> i32 {
        match self {
            Exit::Ok => 0,
            Exit::Err(err) => {
                eprintln!("Error: {:?}", err);
                err.into()
            },
        }
    }
}

impl<T> Try for Exit<T> {
    type Ok = ();
    type Error = T;

    fn into_result(self) -> Result<<Exit<T> as Try>::Ok, Self::Error> {
        match self {
            Exit::Ok => Ok(()),
            Exit::Err(err) => Err(err)
        }
    }

    fn from_error(err: Self::Error) -> Self {
        Exit::Err(err)
    }

    fn from_ok(_: <Exit<T> as Try>::Ok) -> Self {
        Exit::Ok
    }
}
