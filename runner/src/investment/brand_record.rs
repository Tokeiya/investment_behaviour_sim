use crate::argument_error::ArgumentError;
use crate::invalid_operation_error::InvalidOperationError;

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

	pub fn sharp_ratio(&self, rf: f64) -> Result<f64, InvalidOperationError> {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_test() {
		let fixture = BrandRecord::new("test".to_string(), 0.25, 0.5).unwrap();
		assert_eq!(fixture.name, "test");
		assert_eq!(fixture.mean_interest, 0.25);
		assert_eq!(fixture.deviation, 0.5);

		let fixture = BrandRecord::new("test".to_string(), 0.25, 0.0).unwrap();
		assert_eq!(fixture.name, "test");
		assert_eq!(fixture.mean_interest, 0.25);
		assert_eq!(fixture.deviation, 0.0);

		let fixture = BrandRecord::new("test".to_string(), 0.0, -0.25)
			.err()
			.unwrap();
		assert!(matches!(fixture, ArgumentError::ArgumentOutOfRange(_)));
	}

	#[test]
	fn name_test() {
		let fixture = BrandRecord::new("test".to_string(), 0.25, 0.5).unwrap();
		assert_eq!(fixture.name(), "test");
	}

	#[test]
	fn mean_interest_test() {
		let fixture = BrandRecord::new("test".to_string(), 0.25, 0.5).unwrap();
		assert_eq!(fixture.mean_interest(), 0.25);
	}

	#[test]
	fn deviation_test() {
		let fixture = BrandRecord::new("test".to_string(), 0.25, 0.5).unwrap();
		assert_eq!(fixture.deviation(), 0.5);
	}

	#[test]
	fn sharp_ratio_test() {
		todo!()
	}
}
