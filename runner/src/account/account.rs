use super::passbook::PassBook;
use super::transaction_record::TransactionRecord;
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

	pub fn transaction(&mut self, amount: Decimal) -> TransactionRecord {
		self.n += 1;
		self.current_balance += amount;
		let record = TransactionRecord::new(self.n, amount, self.current_balance);

		self.transaction_history.add_transaction(record);

		record
	}

	pub fn current_balance(&self) -> Decimal {
		self.current_balance
	}

	pub fn passbook(&self) -> &PassBook {
		&self.transaction_history
	}

	pub fn n(&self) -> usize {
		self.n
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_test() {
		let account = Account::new();
		assert_eq!(account.current_balance(), Decimal::ZERO);
		assert_eq!(account.n, 0);
		assert_eq!(account.passbook().transaction_history().len(), 0);
	}

	#[test]
	fn transaction_test() {
		let mut account = Account::new();
		let record = account.transaction(Decimal::from(100));
		assert_eq!(account.current_balance(), Decimal::from(100));
		assert_eq!(account.n, 1);
		assert_eq!(account.passbook().transaction_history().len(), 1);

		assert_eq!(record.n(), 1);
		assert_eq!(record.amount(), Decimal::from(100));
		assert_eq!(record.balance(), Decimal::from(100));

		let record = account.transaction(Decimal::from(-100));
		assert_eq!(account.current_balance(), Decimal::ZERO);
		assert_eq!(account.n, 2);
		assert_eq!(account.passbook().transaction_history().len(), 2);

		assert_eq!(record.n(), 2);
		assert_eq!(record.amount(), Decimal::from(-100));
		assert_eq!(record.balance(), Decimal::ZERO);
	}

	#[test]
	fn passbook_test() {
		let mut account = Account::new();
		account.transaction(Decimal::from(100));
		account.transaction(Decimal::from(-100));

		assert_eq!(account.passbook().transaction_history().len(), 2);
		let fixture = account.passbook();

		assert_eq!(fixture.transaction_history().len(), 2);
		assert_eq!(fixture.transaction_history()[0].n(), 1);
		assert_eq!(
			fixture.transaction_history()[0].amount(),
			Decimal::from(100)
		);
		assert_eq!(
			fixture.transaction_history()[0].balance(),
			Decimal::from(100)
		);
		assert_eq!(fixture.transaction_history()[1].n(), 2);
		assert_eq!(
			fixture.transaction_history()[1].amount(),
			Decimal::from(-100)
		);
		assert_eq!(fixture.transaction_history()[1].balance(), Decimal::ZERO);
	}
}
