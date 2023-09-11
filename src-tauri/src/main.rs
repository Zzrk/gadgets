#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use enigo::*;

#[tauri::command]
fn mousescroll() {
    // 鼠标滚轮事件
    println!("mousescroll");
    let mut enigo: Enigo = Enigo::new();
    enigo.mouse_scroll_y(10);
}

#[tauri::command]
fn mousemove() {
    // 鼠标移动和点击事件
    println!("mousemove");
    let mut enigo = Enigo::new();
    // TODO: 下一章按钮的位置不固定，用键盘代替
    enigo.mouse_move_to(1250, 1250);
    enigo.mouse_click(MouseButton::Left);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![mousescroll, mousemove])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
