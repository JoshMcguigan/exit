#![feature(termination_trait_lib)]
#![feature(try_trait)]

use std::process::Termination;
use std::fmt::Debug;
use std::ops::Try;

pub enum Exit<T> {
    Ok,
    Err(T)
}

pub enum ExitWithMessage<T> {
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
            },
        }
    }
}

// todo rather than requiring string, allow anything which impls display
impl<T: Into<(i32, String)>> Termination for ExitWithMessage<T> {
    fn report(self) -> i32 {
        match self {
            ExitWithMessage::Ok => 0,
            ExitWithMessage::Err(err) => {
                let (exit_code, exit_message) = err.into();
                eprintln!("{}", exit_message);
                exit_code
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

impl<T> Try for ExitWithMessage<T> {
    type Ok = ();
    type Error = T;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self {
            ExitWithMessage::Ok => Ok(()),
            ExitWithMessage::Err(err) => Err(err)
        }
    }

    fn from_error(err: Self::Error) -> Self {
        ExitWithMessage::Err(err)
    }

    fn from_ok(_: Self::Ok) -> Self {
        ExitWithMessage::Ok
    }
}
