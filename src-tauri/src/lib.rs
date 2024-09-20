use std::collections::HashMap;

use oauth::{github, utils::extract_provider_and_code};
use serde::{Deserialize, Serialize};
use tauri::{async_runtime, Manager, WebviewWindow};
use tauri_plugin_deep_link::DeepLinkExt;

mod commands;
mod config;
mod oauth;
mod utils;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payload<'a> {
    message: &'a str,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_single_instance::init(|handle, args, _cwd| {
            let handle = handle.clone();
            let windows: HashMap<String, WebviewWindow> = handle.webview_windows();

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
                                if let Err(err) =
                                    github::exchange_code_for_token(&handle, code).await
                                {
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

            #[cfg(debug_assertions)]
            {
                window.open_devtools();
                app.deep_link()
                    .register("keysync")
                    .expect("Failed to register deep link");
            }

            let is_first_launch = utils::create_config_file_if_not_exists();

            if is_first_launch {
                println!("First launch detected, opening login page");
                window.eval("window.location.replace(\"/login\");").unwrap();
            } else {
                println!("Not the first launch, proceeding to app"); // Todo: instead of first launch it should be whether the user is logged in or not
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::set_config,
            commands::github_login,
            commands::get_github_user_info,
            commands::discord_login,
            commands::google_login,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}
