// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize)]
struct TodoItem {
    id: u64,
    description: String,
    deadline: u64,
    completed: bool,
}

#[tauri::command]
fn load_tasks() -> Result<Vec<TodoItem>, String> {
    let path = "/Users/will/code/yo-tasks/Dropbox/tasks.json";
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(e.to_string()),
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        return Err(e.to_string());
    }

    let todos = match serde_json::from_str(&contents) {
        Ok(todos) => todos,
        Err(e) => return Err(e.to_string()),
    };

    Ok(todos)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_tasks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
