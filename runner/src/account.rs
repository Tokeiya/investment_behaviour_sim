use crate::passbook::PassBook;
use crate::transaction_record::TransactionRecord;
use rust_decimal::Decimal;

pub struct Account {
	n: usize,
	current_balance: Decimal,
	transaction_history: PassBook,
}

impl Account {
	pub fn new() -> Self {
		Self {
			n: 0,
			current_balance: Decimal::ZERO,
			transaction_history: PassBook::new(),
		}
	}

	pub fn transaction(&mut self, amount: Decimal) {
		self.current_balance += amount;
		self.transaction_history
			.add_transaction(TransactionRecord::new(self.n, amount, self.current_balance));
		self.n += 1;
	}

	pub fn current_balance(&self) -> Decimal {
		self.current_balance
	}

	pub fn passbook(&self) -> &PassBook {
		&self.transaction_history
	}
}
