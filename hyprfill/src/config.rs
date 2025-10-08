use super::*;
use std::{env, fs};

const DEFAULT_CONFIG_PATH: &str = "$HOME/.config/hypr/hyprfill.json";

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Config {
    pub defaultcommand: Option<Vec<String>>,
    pub workspaces: Vec<WorkspaceConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkspaceConfig {
    pub id: usize,
    pub monitorbyid: Option<usize>,
    pub monitorbyname: Option<String>,
    pub monitorbydesc: Option<String>,
    pub commands: Option<Vec<String>>,
}

impl Config {
    fn resolve_config_path(config_path: Option<String>) -> Result<String> {
        let config_path = match config_path {
            Some(path) => path,
            None => {
                let home_dir = env::home_dir().ok_or(FillError::NoHomeDir)?;
                let home_dir = home_dir.to_str().unwrap();
                DEFAULT_CONFIG_PATH.replace("$HOME", home_dir)
            }
        };

        Ok(config_path)
    }

    pub fn load_config(config_path: Option<String>) -> Result<Config> {
        let config_path = Self::resolve_config_path(config_path)?;

        let data = fs::read_to_string(config_path)?;
        let config = serde_json::from_str(&data)?;

        Ok(config)
    }

    pub fn setup_with_default_config(config_path: Option<String>) -> Result<Option<String>> {
        let config_path = Self::resolve_config_path(config_path)?;

        if !fs::exists(&config_path)? {
            let data = serde_json::to_string_pretty(&Config::default())?;
            fs::write(&config_path, data)?;
            Ok(Some(config_path))
        } else {
            Ok(None)
        }
    }

    pub fn example_config() -> Result<String> {
        let example_config = Self {
            defaultcommand: Some(vec!["sinkgui".to_owned()]),
            workspaces: vec![
                WorkspaceConfig {
                    id: 1,
                    monitorbyid: Some(2),
                    monitorbyname: None,
                    monitorbydesc: None,
                    commands: None,
                },
                WorkspaceConfig {
                    id: 2,
                    monitorbyid: None,
                    monitorbyname: Some("eDP-1".to_owned()),
                    monitorbydesc: None,
                    commands: Some(vec![
                        "alacritty".to_owned(),
                        "-T".to_owned(),
                        "hello".to_owned(),
                    ]),
                },
                WorkspaceConfig {
                    id: 3,
                    monitorbyid: None,
                    monitorbyname: None,
                    monitorbydesc: Some("Microstep MSI MP275Q PC3M255201432".to_owned()),
                    commands: None,
                },
            ],
        };
        let data = serde_json::to_string_pretty(&example_config)?;

        Ok(data)
    }
}
