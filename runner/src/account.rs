use crate::transaction_record::TransactionRecord;
use rust_decimal::Decimal;

pub struct Account {
	n: usize,
	current_balance: Decimal,
	transaction_history: Vec<TransactionRecord>,
}

impl Account {
	pub fn new() -> Self {
		Self {
			n: 0,
			current_balance: Decimal::ZERO,
			transaction_history: Vec::new(),
		}
	}

	pub fn transaction(&mut self, amount: Decimal) {
		self.current_balance += amount;
		self.transaction_history
			.push(TransactionRecord::new(self.n, amount, self.current_balance));
		self.n += 1;
	}

	pub fn current_balance(&self) -> Decimal {
		self.current_balance
	}

	pub fn transaction_history(&self) -> &[TransactionRecord] {
		&self.transaction_history
	}
}
