// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app as lib;
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let icon = app
                .default_window_icon()
                .cloned()
                .expect("missing configured application icon");

            TrayIconBuilder::with_id("main")
                .icon(icon)
                .icon_as_template(true)
                .on_tray_icon_event(|_tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        println!("I'm clicked");
                    }
                })
                .build(app)?;

            #[cfg(target_os = "macos")]
            app.handle()
                .set_activation_policy(tauri::ActivationPolicy::Accessory)?;

            let _profile = lib::load_configured_profile()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
