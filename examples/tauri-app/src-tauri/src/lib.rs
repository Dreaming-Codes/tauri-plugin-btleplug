use tauri_plugin_btleplug::BtleplugExt;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .plugin(tauri_plugin_btleplug::init())
        .setup(|app| {
            #[cfg(mobile)]
            app.btleplug().request_permissions(tauri_plugin_btleplug::permission::RequestPermission {
                bluetooth: true,
                bluetooth_admin: true,
                bluetooth_advertise: true,
                bluetooth_connect: true,
                bluetooth_scan: true,
            }).expect("error while requesting permissions");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
