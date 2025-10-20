use super::transaction_record::TransactionRecord;
use rust_decimal::Decimal;
type TransactionHistory = Vec<TransactionRecord>;
pub struct Account {
	n: usize,
	transaction_history: TransactionHistory,
}

impl Account {
	pub fn new() -> Self {
		Self {
			n: 0,
			transaction_history: TransactionHistory::new(),
		}
	}

	pub fn transaction(&mut self, amount: Decimal) -> TransactionRecord {
		self.n += 1;

		let record = TransactionRecord::new(self.n, amount, self.current_balance() + amount);
		self.transaction_history.push(record);

		record
	}

	pub fn current_balance(&self) -> Decimal {
		let recent = self.transaction_history.last();

		if let Some(recent) = recent {
			return recent.balance();
		} else {
			return Decimal::ZERO;
		}
	}

	pub fn transaction_history(&self) -> &[TransactionRecord] {
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
		assert_eq!(account.transaction_history().len(), 0);
	}

	#[test]
	fn transaction_test() {
		let mut account = Account::new();
		let record = account.transaction(Decimal::from(100));
		assert_eq!(account.current_balance(), Decimal::from(100));
		assert_eq!(account.n, 1);
		assert_eq!(account.transaction_history().len(), 1);

		assert_eq!(record.n(), 1);
		assert_eq!(record.amount(), Decimal::from(100));
		assert_eq!(record.balance(), Decimal::from(100));

		let record = account.transaction(Decimal::from(-100));
		assert_eq!(account.current_balance(), Decimal::ZERO);
		assert_eq!(account.n, 2);
		assert_eq!(account.transaction_history().len(), 2);

		assert_eq!(record.n(), 2);
		assert_eq!(record.amount(), Decimal::from(-100));
		assert_eq!(record.balance(), Decimal::ZERO);
	}

	#[test]
	fn transaction_history_test() {
		let mut account = Account::new();
		account.transaction(Decimal::from(100));
		account.transaction(Decimal::from(-100));

		assert_eq!(account.transaction_history().len(), 2);
		let fixture = account.transaction_history();

		assert_eq!(fixture.len(), 2);
		assert_eq!(fixture[0].n(), 1);
		assert_eq!(fixture[0].amount(), Decimal::from(100));
		assert_eq!(fixture[0].balance(), Decimal::from(100));
		assert_eq!(fixture[1].n(), 2);
		assert_eq!(fixture[1].amount(), Decimal::from(-100));
		assert_eq!(fixture[1].balance(), Decimal::ZERO);
	}
}
