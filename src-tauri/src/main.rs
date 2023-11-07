// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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

const TASKS_PATH: &str = "/Users/will/code/yo-tasks/Dropbox/tasks.json";

#[tauri::command]
fn load_tasks() -> Result<Vec<Task>, String> {
    let mut file = match File::open(TASKS_PATH) {
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

#[tauri::command]
fn save_tasks(tasks: Vec<Task>) -> Result<(), String> {
    let serialized_tasks = serde_json::to_string(&tasks).map_err(|e| e.to_string())?;

    let mut file = File::create(TASKS_PATH).map_err(|e| e.to_string())?;
    file.write_all(serialized_tasks.as_bytes())
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_tasks, save_tasks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
