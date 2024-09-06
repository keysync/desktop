use std::{fs::{self, File}, io::Write};

use config::Config;
use dirs::home_dir;
use oauth::{github, utils::extract_provider_and_code};
use tauri::{async_runtime, Manager};
use tauri_plugin_deep_link::DeepLinkExt;

mod commands;
mod config;
mod oauth;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            let windows: std::collections::HashMap<String, tauri::WebviewWindow> = app.webview_windows();

            // Find and focus the main window
            windows
                .values()
                .next()
                .expect("No window found")
                .set_focus()
                .expect("Failed to set focus");

            // Extract deep link from args
            // keysync://auth/github/callback?code=5e03afbcc10d6b83c46e
            if let Some(link) = args.iter().find(|arg| arg.starts_with("keysync://auth/")) {
                if let Some((provider, code)) = extract_provider_and_code(link) {
                    println!("Provider: {}, Code: {}", provider, code);

                    match provider.as_str() {
                        "github" => {
							async_runtime::spawn(async move {
                                if let Err(err) = github::exchange_code_for_token(code).await {
                                    println!("Failed to exchange code for token: {}", err);
                                }
                            });
						}
                        "discord" => {}
                        "google" => {}
                        _ => println!("Unknown provider: {}", provider),
                    }
                }
            }
        }))
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            if cfg!(debug_assertions) {
                window.open_devtools();
                app.deep_link()
                    .register("keysync")
                    .expect("Failed to register deep link");
            }

			let folder_name = if cfg!(debug_assertions) {
				".keysync-dev"
			} else {
				".keysync"
			};

			if let Some(home_path) = home_dir() {
                let folder_path = home_path.join(folder_name);
                if !folder_path.exists() {
					println!("Directory does not exist, creating it");
                    fs::create_dir(&folder_path).expect("Failed to create directory");
                }

				let config_path = folder_path.join("config.json");
				if !config_path.exists() {
					println!("Config file does not exist, creating it");
					let default_config = Config {
						..Default::default()
					};
					let config_json = serde_json::to_string_pretty(&default_config).expect("Failed to serialize default config");
					let mut file = File::create(&config_path).expect("Failed to create config file");
					file.write_all(config_json.as_bytes()).expect("Failed to write to config file");
				}
            } else {
                eprintln!("Failed to get home directory");
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
			commands::get_config,
			commands::set_config,
            commands::github_login,
            commands::discord_login,
            commands::google_login,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}
