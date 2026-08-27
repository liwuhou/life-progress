#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread, time::Duration};

use app as lib;
use chrono::{Local, NaiveDate};
use tauri::{
    menu::{MenuBuilder, MenuItem, MenuItemBuilder},
    tray::TrayIconBuilder,
    AppHandle, Manager, Runtime, WebviewUrl, WebviewWindowBuilder,
};

struct SummaryMenuItems<R: Runtime> {
    elapsed: MenuItem<R>,
    remaining: MenuItem<R>,
    percent: MenuItem<R>,
}

fn open_settings<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    if let Some(window) = app.get_webview_window("settings") {
        window.show()?;
        window.set_focus()?;
        return Ok(());
    }
    let window = WebviewWindowBuilder::new(app, "settings", WebviewUrl::App("index.html".into()))
        .title("Life Progress 设置")
        .inner_size(520.0, 680.0)
        .resizable(false)
        .build()?;
    let close_window = window.clone();
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            let _ = close_window.hide();
        }
    });
    Ok(())
}

fn summary_texts(state: Option<&lib::DisplayState>) -> (String, String, String) {
    match state {
        Some(state) => (
            format!("已过：{} 天", state.elapsed_days),
            format!("剩余：{} 天", state.remaining_days),
            format!("进度：{}%", state.percent),
        ),
        None => (
            "已过：—".to_string(),
            "剩余：—".to_string(),
            "进度：—".to_string(),
        ),
    }
}

fn set_menu_summary<R: Runtime>(
    app: &AppHandle<R>,
    state: Option<&lib::DisplayState>,
) -> Result<(), String> {
    let items = app.state::<SummaryMenuItems<R>>();
    let (elapsed, remaining, percent) = summary_texts(state);
    items
        .elapsed
        .set_text(elapsed)
        .map_err(|e| format!("update elapsed summary: {e}"))?;
    items
        .remaining
        .set_text(remaining)
        .map_err(|e| format!("update remaining summary: {e}"))?;
    items
        .percent
        .set_text(percent)
        .map_err(|e| format!("update percent summary: {e}"))?;
    Ok(())
}

fn clear_tray<R: Runtime>(app: &AppHandle<R>) {
    let _ = set_menu_summary(app, None);
    if let Some(tray) = app.tray_by_id("main") {
        let _ = tray.set_title::<&str>(None);
        let _ = tray.set_icon(app.default_window_icon().cloned());
    }
}

fn refresh_tray<R: Runtime>(
    app: &AppHandle<R>,
    profile: &life_progress_core::Profile,
    preferences: &lib::DesktopPreferences,
) -> Result<(), String> {
    let state = lib::display_state(profile, preferences).map_err(|e| e.to_string())?;
    let tray = app
        .tray_by_id("main")
        .ok_or_else(|| "main tray icon is unavailable".to_string())?;
    set_menu_summary(app, Some(&state))?;
    tray.set_title(Some(&state.title))
        .map_err(|e| e.to_string())?;
    let icon = if preferences.style.is_graphic() {
        let (rgba, width, height) = lib::render_progress_icon(
            state.percent,
            &preferences.style,
            preferences.icon_width,
            &preferences.color_mode,
            &preferences.threshold_boundaries,
            preferences.border_radius,
        );
        tauri::image::Image::new_owned(rgba, width, height)
    } else {
        app.default_window_icon()
            .cloned()
            .ok_or_else(|| "missing configured application icon".to_string())?
    };
    tray.set_icon(Some(icon)).map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RefreshOutcome {
    Updated,
    NeedsSetup,
    Unavailable,
}

fn refresh_from_disk<R: Runtime>(app: &AppHandle<R>) -> RefreshOutcome {
    match lib::load_configured_profile() {
        Ok(Some(profile)) => match lib::load_preferences() {
            Ok(preferences) => {
                if refresh_tray(app, &profile, &preferences).is_ok() {
                    RefreshOutcome::Updated
                } else {
                    clear_tray(app);
                    RefreshOutcome::Unavailable
                }
            }
            Err(_) => {
                clear_tray(app);
                RefreshOutcome::Unavailable
            }
        },
        Ok(None) => {
            clear_tray(app);
            RefreshOutcome::NeedsSetup
        }
        Err(_) => {
            clear_tray(app);
            RefreshOutcome::Unavailable
        }
    }
}
fn should_advance_date(previous: NaiveDate, current: NaiveDate, refresh_succeeded: bool) -> bool {
    date_changed(previous, current) && refresh_succeeded
}

fn date_changed(previous: NaiveDate, current: NaiveDate) -> bool {
    previous != current
}

fn start_date_refresh<R: Runtime>(app: AppHandle<R>) {
    thread::spawn(move || {
        let mut last_date: NaiveDate = Local::now().date_naive();
        loop {
            thread::sleep(Duration::from_secs(60));
            let current_date = Local::now().date_naive();
            if date_changed(last_date, current_date) {
                let refresh_succeeded = matches!(
                    refresh_from_disk(&app),
                    RefreshOutcome::Updated | RefreshOutcome::NeedsSetup
                );
                if should_advance_date(last_date, current_date, refresh_succeeded) {
                    last_date = current_date;
                }
            }
        }
    });
}

#[tauri::command]
fn get_settings() -> Result<(Option<life_progress_core::Profile>, lib::DesktopPreferences), String>
{
    lib::get_settings()
}

#[tauri::command]
fn save_settings(
    app: AppHandle,
    birthday: String,
    gender: Option<life_progress_core::Gender>,
    nation: String,
    preferences: lib::DesktopPreferences,
) -> Result<life_progress_core::Profile, String> {
    let profile = lib::save_settings(birthday, gender, nation, preferences.clone())?;
    refresh_tray(&app, &profile, &preferences)?;
    Ok(profile)
}

#[tauri::command]
fn search_nations(query: String) -> Result<Vec<String>, String> {
    lib::search_nations(query)
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TrayMenuEntry {
    Item(&'static str, &'static str, bool),
    Separator,
}

fn tray_menu_layout() -> [TrayMenuEntry; 6] {
    [
        TrayMenuEntry::Item("summary-elapsed", "已过：—", false),
        TrayMenuEntry::Item("summary-remaining", "剩余：—", false),
        TrayMenuEntry::Item("summary-percent", "进度：—", false),
        TrayMenuEntry::Separator,
        TrayMenuEntry::Item("settings", "设置", true),
        TrayMenuEntry::Item("quit", "退出", true),
    ]
}
fn menu_item_spec(entry: TrayMenuEntry) -> (&'static str, &'static str, bool) {
    match entry {
        TrayMenuEntry::Item(id, text, enabled) => (id, text, enabled),
        TrayMenuEntry::Separator => panic!("separator is not a menu item"),
    }
}

fn build_tray_menu<R: Runtime>(
    app: &AppHandle<R>,
) -> tauri::Result<(tauri::menu::Menu<R>, SummaryMenuItems<R>)> {
    let layout = tray_menu_layout();
    assert_eq!(layout[3], TrayMenuEntry::Separator);
    let (elapsed_id, elapsed_text, elapsed_enabled) = menu_item_spec(layout[0]);
    let (remaining_id, remaining_text, remaining_enabled) = menu_item_spec(layout[1]);
    let (percent_id, percent_text, percent_enabled) = menu_item_spec(layout[2]);
    let (settings_id, settings_text, settings_enabled) = menu_item_spec(layout[4]);
    let (quit_id, quit_text, quit_enabled) = menu_item_spec(layout[5]);
    let elapsed = MenuItemBuilder::with_id(elapsed_id, elapsed_text)
        .enabled(elapsed_enabled)
        .build(app)?;
    let remaining = MenuItemBuilder::with_id(remaining_id, remaining_text)
        .enabled(remaining_enabled)
        .build(app)?;
    let percent = MenuItemBuilder::with_id(percent_id, percent_text)
        .enabled(percent_enabled)
        .build(app)?;
    let settings = MenuItemBuilder::with_id(settings_id, settings_text)
        .enabled(settings_enabled)
        .build(app)?;
    let quit = MenuItemBuilder::with_id(quit_id, quit_text)
        .enabled(quit_enabled)
        .build(app)?;
    let menu = MenuBuilder::new(app)
        .item(&elapsed)
        .item(&remaining)
        .item(&percent)
        .separator()
        .item(&settings)
        .item(&quit)
        .build()?;
    Ok((
        menu,
        SummaryMenuItems {
            elapsed,
            remaining,
            percent,
        },
    ))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_settings,
            save_settings,
            search_nations
        ])
        .setup(|app| {
            let icon = app
                .default_window_icon()
                .cloned()
                .expect("missing configured application icon");
            let (menu, summary_items) = build_tray_menu(&app.handle())?;
            app.manage(summary_items);

            let handle = app.handle().clone();
            TrayIconBuilder::with_id("main")
                .icon(icon)
                .show_menu_on_left_click(true)
                .menu(&menu)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "settings" => {
                        let _ = open_settings(app);
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .build(app)?;
            #[cfg(target_os = "macos")]
            app.handle()
                .set_activation_policy(tauri::ActivationPolicy::Accessory)?;

            let needs_setup = matches!(refresh_from_disk(&handle), RefreshOutcome::NeedsSetup);
            if needs_setup {
                open_settings(&handle)?;
            }
            start_date_refresh(handle);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state() -> lib::DisplayState {
        lib::DisplayState {
            percent: 58,
            days: 12_345,
            elapsed_days: 12_345,
            remaining_days: 17_890,
            title: String::new(),
        }
    }

    #[test]
    fn summary_texts_show_both_day_counts_and_selected_percent() {
        assert_eq!(
            summary_texts(Some(&state())),
            (
                "已过：12345 天".to_string(),
                "剩余：17890 天".to_string(),
                "进度：58%".to_string()
            )
        );
    }

    #[test]
    fn unavailable_summary_does_not_reuse_personal_values() {
        assert_eq!(
            summary_texts(None),
            (
                "已过：—".to_string(),
                "剩余：—".to_string(),
                "进度：—".to_string()
            )
        );
    }

    #[test]
    fn date_refresh_only_triggers_when_local_date_changes() {
        let day = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        assert!(!date_changed(day, day));
        assert!(date_changed(day, day.succ_opt().unwrap()));
    }
    #[test]
    fn failed_date_refresh_retries_until_success() {
        let day = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let next_day = day.succ_opt().unwrap();
        assert!(!should_advance_date(day, next_day, false));
        assert!(should_advance_date(day, next_day, true));
    }
    #[test]
    fn tray_menu_layout_separates_disabled_summary_from_actions() {
        assert_eq!(
            tray_menu_layout(),
            [
                TrayMenuEntry::Item("summary-elapsed", "已过：—", false),
                TrayMenuEntry::Item("summary-remaining", "剩余：—", false),
                TrayMenuEntry::Item("summary-percent", "进度：—", false),
                TrayMenuEntry::Separator,
                TrayMenuEntry::Item("settings", "设置", true),
                TrayMenuEntry::Item("quit", "退出", true),
            ]
        );
    }
}
