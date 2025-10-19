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
