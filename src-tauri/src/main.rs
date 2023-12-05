// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod recorder;
mod tray;

use std::sync::Arc;
use tauri::{GlobalShortcutManager, Manager};
use tauri_plugin_autostart::MacosLauncher;
use tokio::sync::Mutex;

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

pub enum Status {
    Idle,
    Stage,
    Recording,
    Editing,
}

pub struct AppState {
    status: Status,
    recorder: Arc<Mutex<recorder::Recorder>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            status: Status::Idle,
            recorder: Arc::new(Mutex::new(recorder::new())),
        }
    }
}

const SHORTCUT: &str = "CmdOrCtrl+Shift+2";

fn main() {
    // Set up Tauri Plugins
    let tp_store = tauri_plugin_store::Builder::default().build();
    let tp_single_instance = tauri_plugin_single_instance::init(|_, _, _| {});
    let tp_autostart = tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None);

    tauri::Builder::default()
        .plugin(tp_store)
        .plugin(tp_autostart)
        .plugin(tp_single_instance)
        .setup(|app| {
            // Set activation policy to Accessory on macOS
            #[cfg(target_os = "macos")]
            app.set_activation_policy(ActivationPolicy::Accessory);

            let app_handle = app.app_handle();

            let mut shortcuts = app_handle.global_shortcut_manager();
            if !shortcuts.is_registered(SHORTCUT).unwrap() {
                shortcuts
                    .register(SHORTCUT, move || {
                        println!("TODO: trigger recording");
                    })
                    .unwrap();
            }

            Ok(())
        })
        .manage(Mutex::new(AppState::default()))
        .system_tray(tray::build())
        .on_system_tray_event(tray::events)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}