#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use enigo::*;

#[tauri::command]
fn mousemove() {
    println!("mousemove");
    let mut enigo = Enigo::new();
    enigo.mouse_move_relative(100, 100);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![mousemove])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
