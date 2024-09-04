use tauri::Manager;

mod commands;
mod oauth;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
		.plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let windows = app.webview_windows();
            windows
                .values()
                .next()
                .expect("No window found")
                .set_focus()
                .expect("Failed to set focus");
        }))
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            #[cfg(debug_assertions)]
            {
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::github_login,
            commands::discord_login,
            commands::google_login,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}
