use aes_gcm::{
	aead::{rand_core::RngCore, Aead, KeyInit, OsRng},
	Aes256Gcm,
};
use argon2::Argon2;

pub fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; 32], String> {
	let mut encryption_key = [0u8; 32];
	Argon2::default()
		.hash_password_into(password.as_bytes(), salt, &mut encryption_key)
		.map_err(|e| e.to_string())?;
	Ok(encryption_key)
}

const NONCE_SIZE: usize = 12;

pub fn encrypt(data: &str, key: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	let cipher = Aes256Gcm::new_from_slice(key).map_err(|e| e.to_string())?;

	let mut nonce = [0u8; NONCE_SIZE];
	OsRng.fill_bytes(&mut nonce);

	let encrypted_data = cipher
		.encrypt(&nonce.into(), data.as_ref())
		.map_err(|e| e.to_string())?;
	let mut buffer = Vec::with_capacity(nonce.len() + encrypted_data.len());
	buffer.extend_from_slice(&nonce);
	buffer.extend_from_slice(&encrypted_data);

	Ok(buffer)
}

pub fn decrypt(encrypted_data: &[u8], key: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
	if encrypted_data.len() < NONCE_SIZE {
		return Err("Encrypted data is too short".into());
	}

	let (nonce, ciphertext) = encrypted_data.split_at(NONCE_SIZE);
	let cipher = Aes256Gcm::new_from_slice(key).map_err(|e| e.to_string())?;

	let decrypted_data = cipher
		.decrypt(nonce.into(), ciphertext)
		.map_err(|e| e.to_string())?;
	Ok(String::from_utf8(decrypted_data)?)
}
