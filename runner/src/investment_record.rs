use crate::argument_error::ArgumentError;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct InvestmentRecord {
	n: usize,
	name: String,
	current_interest: f64,
}

impl InvestmentRecord {
	pub fn new(
		name: String,
		mean_interest: f64,
		deviation: f64,
		current_interest: f64,
	) -> Result<Self, ArgumentError> {
		if deviation < 0.0 {
			Err(ArgumentError::ArgumentOutOfRange(format!(
				"deviation must be positive: {}",
				deviation
			)))
		} else {
			Ok(Self {
				n: 0,
				name,
				current_interest,
			})
		}
	}

	pub fn name(&self) -> &str {
		self.name.as_str()
	}

	pub fn current_interest(&self) -> f64 {
		self.current_interest
	}
}
