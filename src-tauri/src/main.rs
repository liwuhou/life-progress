// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app as lib;
use tauri::{Icon, SystemTray, SystemTrayEvent};

fn main() {
    let tray = SystemTray::new();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                println!("I'm clicked");
            }
            _ => {}
        })
        .setup(|app| {
            if lib::is_init_done()? {
                // Update system icon
                let icon_path = lib::generate_progree_icon()?;
            } else {
                // Show the init windows
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
