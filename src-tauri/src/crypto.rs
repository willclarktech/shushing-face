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

pub fn encrypt(data: &String, key: &[u8; ENCRYPTION_KEY_SIZE]) -> Result<Vec<u8>, TasksError> {
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
