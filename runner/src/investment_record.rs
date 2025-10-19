use crate::argument_error::ArgumentError;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct InvestmentRecord {
	n: usize,
	name: String,
	mean_interest: f64,
	deviation: f64,
	current_interest: f64,
}

impl InvestmentRecord {
	pub fn new(
		name: String,
		mean_interest: f64,
		deviation: f64,
		current_interest: f64,
	) -> Result<Self, ArgumentError> {
		todo!()
	}

	pub fn name(&self) -> &str {
		todo!()
	}

	pub fn mean_interest(&self) -> f64 {
		todo!()
	}

	pub fn deviation(&self) -> f64 {
		todo!()
	}

	pub fn current_interest(&self) -> f64 {
		todo!()
	}
}
