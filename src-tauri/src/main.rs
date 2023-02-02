#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


#[tauri::command]
fn test(name: &str) -> String {
    format!("Test, {}!", name)
}

fn main() {
    tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
