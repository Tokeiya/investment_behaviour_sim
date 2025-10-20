use crate::argument_error::ArgumentError;
use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy)]
pub struct TransactionRecord {
	n: usize,
	amount: Decimal,
	balance: Decimal,
}

impl TransactionRecord {
	pub fn new(n: usize, amount: Decimal, balance: Decimal) -> Self {
		Self { n, amount, balance }
	}

	pub fn n(&self) -> usize {
		self.n
	}

	pub fn amount(&self) -> Decimal {
		self.amount
	}

	pub fn balance(&self) -> Decimal {
		self.balance
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_test() {
		let record = TransactionRecord::new(1, Decimal::from(100), Decimal::from(142));
		assert_eq!(record.n(), 1);
		assert_eq!(record.amount(), Decimal::from(100));
		assert_eq!(record.balance(), Decimal::from(142));
	}

	#[test]
	fn n_test() {
		let fixture = TransactionRecord::new(1, Decimal::from(100), Decimal::from(142));
		assert_eq!(fixture.n(), 1);
	}

	#[test]
	fn amount_test() {
		let fixture = TransactionRecord::new(1, Decimal::from(100), Decimal::from(142));
		assert_eq!(fixture.amount(), Decimal::from(100));
	}

	#[test]
	fn balance_test() {
		let fixture = TransactionRecord::new(1, Decimal::from(100), Decimal::from(142));
		assert_eq!(fixture.balance(), Decimal::from(142));
	}
}
