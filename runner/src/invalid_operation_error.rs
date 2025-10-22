use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
pub struct InvalidOperationError(String);

impl InvalidOperationError {
	pub fn new(message: String) -> Self {
		Self(message)
	}
}

impl Display for InvalidOperationError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "InvalidOperation:{}", self.0)
	}
}
