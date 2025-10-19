use thiserror::Error;

#[derive(Error, Debug)]
pub enum AccountError {
	#[error("Insufficient funds")]
	InsufficientFunds,
}
