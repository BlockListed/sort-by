use std::fmt::Display;
use std::io;
use std::process::{Termination, ExitCode};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MainError {
	#[error("Missing argument: {0}")]
	MissingArguments(&'static str),
	#[error("Argument is invalid: {0}")]
	InvalidArgument(&'static str),
	#[error("Output I/O Error")]
	Output(#[from] io::Error),
}

// This exists because the default error for Missing free argument, doesn't (and can't) include which free standing argument is missing.
pub trait ArgumentIntoMain<T> {
	fn into_main(self, argument: &'static str) -> Result<T, MainError>;
}

impl<T> ArgumentIntoMain<T> for Result<T, pico_args::Error> {
	fn into_main(self, argument: &'static str) -> Result<T, MainError> {
		match self {
			Ok(x) => Ok(x),
			Err(e) => {
				use pico_args::Error::{MissingArgument, MissingOption};
				match e {
					MissingArgument | MissingOption(_) => Err(MainError::MissingArguments(argument)),
					_ => Err(MainError::InvalidArgument(argument))
				}
			}
		}
	}
}

// This exists because of limitations on implementing traits on foreign types.
pub trait IntoMainResult {
	fn into_main(self) -> Result<(), MainError>;
}

impl IntoMainResult for Result<(), OutputError> {
	fn into_main(self) -> Result<(), MainError> {
		if let Err(e) = self {
			match e {
				OutputError::Closed => {
					Ok(())
				}
				OutputError::Other(e) => {
					Err(MainError::Output(e))
				}
			}
		} else {
			Ok(())
		}
	}
		
}

#[derive(Error, Debug)]
pub enum OutputError {
	#[error("Output closed, this is probably normal.")]
	Closed,
	#[error("io error: {0}")]
	Other(io::Error),
}

impl From<io::Error> for OutputError {
	fn from(value: io::Error) -> Self {
		match value.kind() {
			io::ErrorKind::BrokenPipe => {
				OutputError::Closed
			}
			_ => {
				OutputError::Other(value)
			}
		}
	}
}

// Because I want to implement a custom termination trait, which uses Display instead of Debug
pub enum MyResult<T, E> {
	Ok(T),
	Err(E),
}

impl<T, E> From<Result<T, E>> for MyResult<T, E> {
	fn from(value: Result<T, E>) -> Self {
		match value {
			Ok(t) => MyResult::Ok(t),
			Err(e) => MyResult::Err(e),
		}
	}
}

impl<T: Termination, E: Display> Termination for MyResult<T, E> {
	fn report(self) -> std::process::ExitCode {
		match self {
			MyResult::Ok(t) => t.report(),
			MyResult::Err(e) => {
				println!("{}", e);
				println!("{}", crate::strings::HELP);
				ExitCode::FAILURE
			}
		}
	}
}

#[macro_export]
macro_rules! my_try {
	($t:expr) => {
		match $t.into() {
			errors::MyResult::Ok(t) => t,
			errors::MyResult::Err(e) => {
				return errors::MyResult::Err(e.into());
			}
		}
	};
}