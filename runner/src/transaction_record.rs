use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TransactionRecord {
	n: usize,
	amount: Decimal,
	balance: Decimal,
}

impl TransactionRecord {
	pub fn new(n: usize, amount: Decimal, balance: Decimal) -> Self {
		Self { n, amount, balance }
	}
}
