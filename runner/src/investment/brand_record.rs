use crate::argument_error::ArgumentError;

pub struct BrandRecord {
	name: String,
	mean_interest: f64,
	deviation: f64,
}

impl BrandRecord {
	pub fn new(name: String, mean_interest: f64, deviation: f64) -> Result<Self, ArgumentError> {
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

	pub fn sharp_ratio(&self, rf: f64) -> f64 {
		todo!()
	}
}
