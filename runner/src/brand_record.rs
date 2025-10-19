use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BrandRecord {
	name: String,
	mean_interest: f64,
	deviation: f64,
}

impl BrandRecord {
	pub fn new(name: String, mean_interest: f64, deviation: f64) -> BrandRecord {
		BrandRecord {
			name,
			mean_interest,
			deviation,
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
	pub fn sharp_ratio(&self, rf: f64) -> f64 {
		(self.mean_interest - rf) / self.deviation
	}
}
