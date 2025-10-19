use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
	#[error("IoError:{0}")]
	IoError(std::io::Error),
	#[error("SerializeError:{0}")]
	SerializeError(#[from] serde_json::Error),
}
