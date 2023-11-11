use std::{error::Error, fmt};
use tauri::InvokeError;

#[derive(Debug)]
pub enum TasksError {
	UnknownError(String),
	ExternalError(Box<dyn std::error::Error>),
	CryptoError(String),
	IoError(std::io::Error),
	SerdeError(serde_json::Error),
	Argon2Error(argon2::Error),
	AesGcmError(aes_gcm::Error),
	AesGcmInvalidLengthError(aes_gcm::aes::cipher::InvalidLength),
	Utf8Error(std::string::FromUtf8Error),
}

impl fmt::Display for TasksError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			TasksError::UnknownError(e) => write!(f, "Error: {}", e),
			TasksError::ExternalError(e) => write!(f, "External error: {}", e),
			TasksError::CryptoError(e) => write!(f, "Crypto error: {}", e),
			TasksError::IoError(e) => write!(f, "IO error: {}", e),
			TasksError::SerdeError(e) => write!(f, "Serialization error: {}", e),
			TasksError::Argon2Error(e) => write!(f, "Argon2 error: {}", e),
			TasksError::AesGcmError(e) => write!(f, "AES-GCM error: {}", e),
			TasksError::AesGcmInvalidLengthError(e) => {
				write!(f, "AES-GCM invalid length error: {}", e)
			}
			TasksError::Utf8Error(e) => write!(f, "UTF8 error: {}", e),
		}
	}
}

impl Error for TasksError {}

impl From<String> for TasksError {
	fn from(err: String) -> Self {
		TasksError::UnknownError(err)
	}
}

impl From<Box<dyn std::error::Error>> for TasksError {
	fn from(error: Box<dyn std::error::Error>) -> Self {
		TasksError::ExternalError(error)
	}
}

impl From<std::io::Error> for TasksError {
	fn from(err: std::io::Error) -> Self {
		TasksError::IoError(err)
	}
}

impl From<serde_json::Error> for TasksError {
	fn from(err: serde_json::Error) -> Self {
		TasksError::SerdeError(err)
	}
}

impl From<std::string::FromUtf8Error> for TasksError {
	fn from(error: std::string::FromUtf8Error) -> Self {
		TasksError::Utf8Error(error)
	}
}

impl From<argon2::Error> for TasksError {
	fn from(error: argon2::Error) -> Self {
		TasksError::Argon2Error(error)
	}
}

impl From<aes_gcm::Error> for TasksError {
	fn from(error: aes_gcm::Error) -> Self {
		TasksError::AesGcmError(error)
	}
}

impl From<aes_gcm::aes::cipher::InvalidLength> for TasksError {
	fn from(error: aes_gcm::aes::cipher::InvalidLength) -> Self {
		TasksError::AesGcmInvalidLengthError(error)
	}
}

impl From<TasksError> for InvokeError {
	fn from(error: TasksError) -> Self {
		InvokeError::from(error.to_string())
	}
}
