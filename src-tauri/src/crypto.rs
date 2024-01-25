use aes_gcm::{
	aead::{rand_core::RngCore, Aead, KeyInit, OsRng},
	Aes256Gcm,
};
use argon2::Argon2;
use rand::{thread_rng, Rng};
use std::sync::Mutex;

use crate::error::TasksError;

pub const SALT_SIZE: usize = 16;
pub const ENCRYPTION_KEY_SIZE: usize = 32;
pub const NONCE_SIZE: usize = 12;

#[derive(Default)]
pub struct EncryptionKey(pub Mutex<[u8; ENCRYPTION_KEY_SIZE]>);

pub type Salt = [u8; SALT_SIZE];

pub fn generate_random_bytes(buf: &mut [u8]) -> () {
	let mut rng = thread_rng();
	rng.fill(buf);
}

pub fn derive_key(
	password: &str,
	salt: &[u8; SALT_SIZE],
	encryption_key: &mut [u8; ENCRYPTION_KEY_SIZE],
) -> Result<(), TasksError> {
	Argon2::default().hash_password_into(password.as_bytes(), salt, encryption_key)?;
	Ok(())
}

pub fn encrypt(data: &[u8], key: &[u8; ENCRYPTION_KEY_SIZE]) -> Result<Vec<u8>, TasksError> {
	let cipher = Aes256Gcm::new_from_slice(key)?;

	let mut nonce = [0u8; NONCE_SIZE];
	OsRng.fill_bytes(&mut nonce);

	let encrypted_data = cipher.encrypt(&nonce.into(), data.as_ref())?;
	let mut buffer = Vec::with_capacity(nonce.len() + encrypted_data.len());
	buffer.extend_from_slice(&nonce);
	buffer.extend_from_slice(&encrypted_data);

	Ok(buffer)
}

pub fn decrypt(
	encrypted_data: &[u8],
	key: &[u8; ENCRYPTION_KEY_SIZE],
) -> Result<String, TasksError> {
	if encrypted_data.len() < NONCE_SIZE {
		return Err(TasksError::CryptoError(
			"Encrypted data is too short".into(),
		));
	}

	let (nonce, ciphertext) = encrypted_data.split_at(NONCE_SIZE);
	let cipher = Aes256Gcm::new_from_slice(key)?;

	let decrypted_data = cipher.decrypt(nonce.into(), ciphertext)?;
	Ok(String::from_utf8(decrypted_data)?)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_generate_random_bytes() {
		let mut buffer = [0u8; SALT_SIZE];
		generate_random_bytes(&mut buffer);
		assert_ne!(buffer, [0u8; SALT_SIZE], "Buffer should not be all zeroes.");
	}

	#[test]
	fn test_derive_key() {
		let password = "strong_password";
		let mut salt = [0u8; SALT_SIZE];
		let mut encryption_key = [0u8; ENCRYPTION_KEY_SIZE];
		generate_random_bytes(&mut salt);
		derive_key(password, &salt, &mut encryption_key).expect("Key derivation should succeed.");
		assert_ne!(
			encryption_key, [0u8; ENCRYPTION_KEY_SIZE],
			"Encryption key should not be all zeroes."
		);
	}

	#[test]
	fn test_encrypt_decrypt() {
		let password = "strong_password";
		let mut salt = [0u8; SALT_SIZE];
		let mut encryption_key = [0u8; ENCRYPTION_KEY_SIZE];
		generate_random_bytes(&mut salt);
		derive_key(password, &salt, &mut encryption_key).expect("Key derivation should succeed.");

		let data = "Data to encrypt".as_bytes();
		let encrypted_data = encrypt(data, &encryption_key).expect("Encryption should succeed.");
		assert_ne!(
			encrypted_data, data,
			"Encrypted data should not match original."
		);

		let decrypted_data =
			decrypt(&encrypted_data, &encryption_key).expect("Decryption should succeed.");
		assert_eq!(
			decrypted_data,
			String::from_utf8_lossy(data),
			"Decrypted data should match original."
		);
	}

	#[test]
	fn test_decrypt_with_invalid_data() {
		let encryption_key = [0u8; ENCRYPTION_KEY_SIZE];
		let invalid_data = [0u8; 5]; // Less than NONCE_SIZE
		assert!(
			decrypt(&invalid_data, &encryption_key).is_err(),
			"Decryption should fail with invalid data."
		);
	}
}
