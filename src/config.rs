use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub donut_dir: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        let config_path = dirs::home_dir()
            .map(|p| p.join(".donut.yml"))
            .unwrap_or_else(|| PathBuf::from(".donut.yml"));

        if let Ok(content) = fs::read_to_string(&config_path) {
            serde_yaml::from_str(&content).unwrap_or_default()
        } else {
            Config::default()
        }
    }

    pub fn get_donut_dir(&self) -> PathBuf {
        if let Some(dir) = &self.donut_dir {
            let expanded = Self::expand_tilde(dir);
            if expanded.is_absolute() {
                expanded
            } else {
                std::env::current_dir()
                    .unwrap_or_default()
                    .join(expanded)
            }
        } else {
            dirs::home_dir()
                .map(|p| p.join(".donut"))
                .unwrap_or_else(|| PathBuf::from(".donut"))
        }
    }

    fn expand_tilde(path: &str) -> PathBuf {
        if path.starts_with("~/") {
            if let Some(home) = dirs::home_dir() {
                return home.join(&path[2..]);
            }
        }
        PathBuf::from(path)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self { donut_dir: None }
    }
}
