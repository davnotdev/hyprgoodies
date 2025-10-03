use super::*;

use std::fs;

const STASH_PATH: &str = "/tmp/hyprstash/";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StashedInstance {
    Workspace(StashedWorkspace),
    Monitor(StashedMonitor),
    Everything(StashedFullSession),
}

impl StashedInstance {
    pub fn write(self, name: &str) -> Result<()> {
        Self::setup_directories()?;

        let data = serde_json::to_string(&self)?;
        fs::write(Self::stash_path(name)?, data).map_err(StashError::IOError)?;

        Ok(())
    }

    pub fn new_from_name(name: &str) -> Result<Self> {
        let data = fs::read_to_string(Self::stash_path(name)?)?;
        let stashed = serde_json::from_str(&data)?;
        Ok(stashed)
    }

    pub fn remove_instance(name: &str) {
        if let Ok(path) = Self::stash_path(name) {
            let _ = fs::remove_file(path);
        }
    }

    pub fn remove_all_instances() {
        let _ = fs::remove_dir_all(STASH_PATH);
        let _ = Self::setup_directories();
    }

    fn stash_path(name: &str) -> Result<String> {
        if name.chars().all(char::is_alphanumeric) {
            Ok(format!("{}/{}", STASH_PATH, name))
        } else {
            Err(StashError::BadName.into())
        }
    }

    fn setup_directories() -> Result<()> {
        fs::create_dir_all(STASH_PATH)?;
        Ok(())
    }
}
