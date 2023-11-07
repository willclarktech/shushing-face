// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use aes_gcm::{
    aead::{rand_core::RngCore, Aead, KeyInit, OsRng},
    Aes256Gcm,
};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u64,
    description: String,
    deadline: u64,
    completed: bool,
}

const TASKS_PATH: &str = "/Users/will/code/yo-tasks/Dropbox/.yo/tasks.json";
const NONCE_SIZE: usize = 12;
// TODO: Change this
const ENCRYPTION_KEY: &[u8; 32] = b"your-32-byte-long-encryption-key";

fn encrypt(data: &str, key: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
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

fn decrypt(encrypted_data: &[u8], key: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    if encrypted_data.len() < NONCE_SIZE {
        return Err("Encrypted data is too short".into());
    }

    let (nonce, ciphertext) = encrypted_data.split_at(NONCE_SIZE);
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|e| e.to_string())?;

    let decrypted_data = cipher
        .decrypt(nonce.into(), ciphertext)
        .map_err(|e| e.to_string())?;
    let decrypted_str = String::from_utf8(decrypted_data)?;

    Ok(decrypted_str)
}

#[tauri::command]
fn save_tasks(tasks: Vec<Task>) -> Result<(), String> {
    let serialized_tasks = serde_json::to_string(&tasks).map_err(|e| e.to_string())?;
    let encrypted_data = encrypt(&serialized_tasks, ENCRYPTION_KEY).map_err(|e| e.to_string())?;

    let mut file = File::create(TASKS_PATH).map_err(|e| e.to_string())?;
    file.write_all(&encrypted_data).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn load_tasks() -> Result<Vec<Task>, String> {
    let mut file = File::open(TASKS_PATH).map_err(|e| e.to_string())?;

    let mut encrypted_data = Vec::new();
    file.read_to_end(&mut encrypted_data)
        .map_err(|e| e.to_string())?;

    let tasks_json = decrypt(&encrypted_data, ENCRYPTION_KEY).map_err(|e| e.to_string())?;
    let tasks = serde_json::from_str(&tasks_json).map_err(|e| e.to_string())?;

    Ok(tasks)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_tasks, load_tasks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
