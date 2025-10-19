use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArgumentError {
	#[error("Invalid argument: {0}")]
	InvalidArgument(String),
	#[error("Argument out of range: {0}")]
	ArgumentOutOfRange(String),
}
