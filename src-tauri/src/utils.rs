use std::{fs::{self, File}, io::Write};

use dirs::home_dir;

use crate::config::Config;

pub fn create_config_file_if_not_exists() -> bool {
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
            let default_config = Config::default();
            let config_json = serde_json::to_string_pretty(&default_config)
                .expect("Failed to serialize default config");
            let mut file = File::create(&config_path).expect("Failed to create config file");
            file.write_all(config_json.as_bytes())
                .expect("Failed to write to config file");

            return true; // First launch
        }
    } else {
        eprintln!("Failed to get home directory");
    }

    false // Not the first launch
}
