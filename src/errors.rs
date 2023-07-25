use std::{io, process::ExitCode};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MainError {
	#[error("Missing argument: {0}")]
	MissingArgument(&'static str),
	#[error("Argument is invalid: {0}")]
	InvalidArgument(&'static str),
	#[error("Output I/O Error. {0}")]
	Output(#[from] io::Error),
}

pub fn argerr_transform(name: &'static str) -> impl FnMut(pico_args::Error) -> MainError {
	move |e| {
		use pico_args::Error::{MissingArgument, MissingOption};
		match e {
			MissingArgument | MissingOption(_) => MainError::MissingArgument(name),
			_ => MainError::InvalidArgument(name),
		}
	}
}

pub fn print_error(e: MainError) -> ExitCode {
	println!("{e}");
	if matches!(e, MainError::InvalidArgument(_) | MainError::MissingArgument(_)) {
		println!("{}", crate::strings::HELP);
	}
	ExitCode::FAILURE
}