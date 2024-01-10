// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{PhysicalPosition, SystemTray, SystemTrayEvent};

fn main() {
    let tray = SystemTray::new();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { position, size, .. } => {
                println!("Left click, position:{:?}， size: {:?}", position, size);
                // TODO: generate a new window, used by position & size
                let local_window =
                    tauri::WindowBuilder::new(app, "local", tauri::WindowUrl::App("/".into()));
                let PhysicalPosition { x, y } = position;
                let _ = local_window
                    .position(x, y)
                    .inner_size(400_f64, 300_f64)
                    .build();
            }
            _ => (),
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
