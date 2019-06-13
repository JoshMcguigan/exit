#![feature(termination_trait_lib)]
#![feature(try_trait)]
#![feature(process_exitcode_placeholder)]
#![feature(specialization)]

use std::fmt::Debug;
use std::ops::Try;
use std::process::{ExitCode, Termination};

pub struct ExitStatus {
    pub status_code: i32,
    pub error: String,
}

impl<T: Debug> From<T> for ExitStatus {
    default fn from(t: T) -> ExitStatus {
        ExitStatus {
            status_code: ExitCode::FAILURE.report(),
            error: format!("{:?}", t),
        }
    }
}

impl<T: Into<i32> + Debug> From<T> for ExitStatus {
    default fn from(t: T) -> ExitStatus {
        ExitStatus {
            error: format!("{:?}", t),
            status_code: t.into(),
        }
    }
}

pub enum Exit<T = ExitStatus> {
    Ok,
    Err(T),
}

impl<T: Into<ExitStatus>> Termination for Exit<T> {
    default fn report(self) -> i32 {
        match self {
            Exit::Ok => ExitCode::SUCCESS.report(),
            Exit::Err(err) => {
                let err = err.into();
                eprintln!("Error: {}", err.error);
                err.status_code
            }
        }
    }
}

impl<T> Try for Exit<T> {
    type Ok = ();
    type Error = T;

    default fn into_result(self) -> Result<<Self as Try>::Ok, Self::Error> {
        match self {
            Exit::Ok => Ok(()),
            Exit::Err(err) => Err(err),
        }
    }

    default fn from_error(err: Self::Error) -> Self {
        Exit::Err(err)
    }

    default fn from_ok(_: <Self as Try>::Ok) -> Self {
        Exit::Ok
    }
}
