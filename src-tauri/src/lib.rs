use tauri::Manager;

mod commands;
mod oauth;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
        .expect("error while running tauri application");
}
