// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TodoItem {
    id: u64,
    description: String,
    deadline: u64,
    completed: bool,
}

#[tauri::command]
fn load_tasks() -> Vec<TodoItem> {
    let mut todos = Vec::new();

    // Add some dummy to-do items
    todos.push(TodoItem {
        id: 1,
        description: "Learn Rust".to_string(),
        deadline: 1699311902670,
        completed: false,
    });
    todos.push(TodoItem {
        id: 2,
        description: "Build a Tauri app".to_string(),
        deadline: 1699311902671,
        completed: false,
    });
    todos.push(TodoItem {
        id: 3,
        description: "Integrate with Svelte".to_string(),
        deadline: 1699311902672,
        completed: false,
    });

    // Return the vector of dummy to-dos
    todos
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_tasks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
