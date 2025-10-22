use crate::argument_error::ArgumentError;
use crate::invalid_operation_error::InvalidOperationError;
use rand_distr::Distribution;
use thiserror::Error;

pub fn generate_brand_record<T: Distribution<f64>>(
	rng: &mut T,
	name: String,
	max_interest: f64,
	min_interest: f64,
	max_deviation: f64,
	min_deviation: f64,
) -> Result<BrandRecord, ArgumentError> {
	todo!()
}

#[derive(Debug, Error)]
pub enum SharpeRatioError {
	#[error("InvalidOperationError:{0}")]
	InvalidOperationError(#[from] InvalidOperationError),
	#[error("ArgumentError:{0}")]
	ArgumentError(#[from] ArgumentError),
}

pub struct BrandRecord {
	name: String,
	mean_interest: f64,
	deviation: f64,
}

impl BrandRecord {
	pub fn new(name: String, mean_interest: f64, deviation: f64) -> Result<Self, ArgumentError> {
		if mean_interest.is_nan() || mean_interest.is_infinite() {
			Err(ArgumentError::InvalidArgument(
				"mean_interest is NaN or Inf".to_string(),
			))
		} else if deviation.is_nan() || deviation.is_infinite() {
			Err(ArgumentError::InvalidArgument(
				"deviation is NaN or Inf".to_string(),
			))
		} else if deviation < 0.0 {
			Err(ArgumentError::ArgumentOutOfRange(
				"deviation is negative".to_string(),
			))
		} else {
			Ok(Self {
				name,
				mean_interest,
				deviation,
			})
		}
	}

	pub fn name(&self) -> &str {
		self.name.as_str()
	}

	pub fn mean_interest(&self) -> f64 {
		self.mean_interest
	}

	pub fn deviation(&self) -> f64 {
		self.deviation
	}

	pub fn sharpe_ratio(&self, rf: f64) -> Result<f64, SharpeRatioError> {
		let numerator = self.mean_interest - rf;

		if rf.is_nan() || rf.is_infinite() {
			return Err(SharpeRatioError::ArgumentError(
				ArgumentError::InvalidArgument("rf is NaN or Inf".to_string()),
			));
		} else if self.deviation == 0.0 && numerator == 0.0 {
			Err(SharpeRatioError::InvalidOperationError(
				InvalidOperationError::new("InvalidOperation:SharpeRatio".to_string()),
			))
		} else if self.deviation == 0.0 {
			if numerator.is_sign_negative() {
				Ok(f64::NEG_INFINITY)
			} else {
				Ok(f64::INFINITY)
			}
		} else {
			Ok(numerator / self.deviation)
		}
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

		let fixture = BrandRecord::new("test".to_string(), f64::NAN, 0.0)
			.err()
			.unwrap();
		assert!(matches!(fixture, ArgumentError::InvalidArgument(_)));

		let fixture = BrandRecord::new("test".to_string(), 0.0, f64::NAN)
			.err()
			.unwrap();
		assert!(matches!(fixture, ArgumentError::InvalidArgument(_)));

		let fixture = BrandRecord::new("test".to_string(), f64::INFINITY, 0.0)
			.err()
			.unwrap();
		assert!(matches!(fixture, ArgumentError::InvalidArgument(_)));

		let fixture = BrandRecord::new("test".to_string(), f64::NEG_INFINITY, 0.0)
			.err()
			.unwrap();
		assert!(matches!(fixture, ArgumentError::InvalidArgument(_)));

		let fixture = BrandRecord::new("test".to_string(), 0.0, f64::INFINITY)
			.err()
			.unwrap();
		assert!(matches!(fixture, ArgumentError::InvalidArgument(_)));

		let fixture = BrandRecord::new("test".to_string(), 0.0, f64::NEG_INFINITY)
			.err()
			.unwrap();
		assert!(matches!(fixture, ArgumentError::InvalidArgument(_)));
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
		let fixture = BrandRecord::new("test".to_string(), 0.25, 0.5).unwrap();
		assert_eq!(fixture.sharpe_ratio(0.05).unwrap(), 0.4);

		assert!(matches!(
			fixture.sharpe_ratio(f64::NAN).unwrap_err(),
			SharpeRatioError::ArgumentError(_)
		));
		assert!(matches!(
			fixture.sharpe_ratio(f64::INFINITY).unwrap_err(),
			SharpeRatioError::ArgumentError(ArgumentError::InvalidArgument(_))
		));
		assert!(matches!(
			fixture.sharpe_ratio(f64::NEG_INFINITY).unwrap_err(),
			SharpeRatioError::ArgumentError(ArgumentError::InvalidArgument(_))
		));

		let fixture = BrandRecord::new("test".to_string(), 0.0, 0.0).unwrap();
		assert_eq!(fixture.sharpe_ratio(0.5).unwrap(), f64::NEG_INFINITY);
		assert_eq!(fixture.sharpe_ratio(-0.5).unwrap(), f64::INFINITY);

		assert!(matches!(
			fixture.sharpe_ratio(0.0).unwrap_err(),
			SharpeRatioError::InvalidOperationError(_)
		));
	}
}
