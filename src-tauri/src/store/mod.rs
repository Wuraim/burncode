pub mod routing_analytics;
pub mod settings;

use crate::store::settings::{settings_path, AppSettings};
use std::path::PathBuf;

#[derive(Clone)]
pub struct SettingsStore {
    pub config_dir: PathBuf,
}



impl SettingsStore {
    pub fn new(config_dir: PathBuf) -> Self {
        Self { config_dir }
    }

    pub fn path(&self) -> PathBuf {
        settings_path(self.config_dir.clone())
    }

    pub async fn load(&self) -> Result<AppSettings, String> {
        let path = self.path();
        if !path.exists() {
            return Ok(AppSettings::default());
        }
        let bytes = tokio::fs::read(&path).await.map_err(|e| e.to_string())?;
        let settings = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
        Ok(settings)
    }

    pub async fn save(&self, settings: &AppSettings) -> Result<(), String> {
        tokio::fs::create_dir_all(&self.config_dir)
            .await
            .map_err(|e| e.to_string())?;
        let bytes = serde_json::to_vec_pretty(settings).map_err(|e| e.to_string())?;
        tokio::fs::write(self.path(), bytes)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
