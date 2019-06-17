#![feature(termination_trait_lib)]
#![feature(try_trait)]
#![feature(specialization)]

use std::process::Termination;
use std::ops::Try;

pub enum Exit<T> {
    Ok,
    Err(T)
}

pub trait ExitDisplay {
    fn display(&self) -> String;
}

impl<T: Into<i32>> Termination for Exit<T> {
    default fn report(self) -> i32 {
        match self {
            Exit::Ok => 0,
            Exit::Err(err) => {
                err.into()
            },
        }
    }
}

impl<T: Into<i32> + ExitDisplay> Termination for Exit<T> {
    fn report(self) -> i32 {
        match self {
            Exit::Ok => 0,
            Exit::Err(err) => {
                eprintln!("{}", err.display());
                err.into()
            },
        }
    }
}

impl<T> Try for Exit<T> {
    type Ok = ();
    type Error = T;

    fn into_result(self) -> Result<<Self as Try>::Ok, Self::Error> {
        match self {
            Exit::Ok => Ok(()),
            Exit::Err(err) => Err(err)
        }
    }

    fn from_error(err: Self::Error) -> Self {
        Exit::Err(err)
    }

    fn from_ok(_: <Self as Try>::Ok) -> Self {
        Exit::Ok
    }
}
