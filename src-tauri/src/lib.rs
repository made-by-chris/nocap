use device_query::{DeviceQuery, DeviceState};
use serde::Serialize;

#[derive(Serialize)]
struct CursorPosition {
  x: i32,
  y: i32,
}

#[tauri::command]
fn get_cursor_position() -> CursorPosition {
  let device_state = DeviceState::new();
  let mouse = device_state.get_mouse();
  CursorPosition {
    x: mouse.coords.0,
    y: mouse.coords.1,
  }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  use tauri_plugin_log::{Target, TargetKind};

  tauri::Builder::default()
    .plugin(
      tauri_plugin_log::Builder::default()
        .level(log::LevelFilter::Info)
        .targets([
          Target::new(TargetKind::Stdout),
          Target::new(TargetKind::LogDir { file_name: None }),
          Target::new(TargetKind::Webview),
        ])
        .build(),
    )
    .invoke_handler(tauri::generate_handler![get_cursor_position])
    .setup(|app| {
      use tauri::Manager;

      log::info!("NoCap app booting");

      let enable_devtools = std::env::args().any(|arg| arg == "--devtools")
        || std::env::var("NOCAP_DEVTOOLS").ok().as_deref() == Some("1");
      if enable_devtools {
        log::info!("Devtools requested by runtime flag/env");
        if let Some(window) = app.get_webview_window("main") {
          window.open_devtools();
        }
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
