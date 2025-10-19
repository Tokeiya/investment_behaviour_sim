use super::transaction_record::TransactionRecord;
use rust_decimal::Decimal;
use serde::Serialize;
use std::io::{Result as IoResult, Write};

pub struct PassBook(Vec<TransactionRecord>);

impl PassBook {
	pub fn new() -> Self {
		Self(Vec::new())
	}

	pub fn add_transaction(&mut self, record: TransactionRecord) {
		self.0.push(record);
	}
	pub fn transaction_history(&self) -> &[TransactionRecord] {
		&self.0
	}
}
