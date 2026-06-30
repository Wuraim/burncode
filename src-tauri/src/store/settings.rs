use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectionProfile {
    pub server_url: String,
    pub username: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub auto_connect: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppSettings {
    #[serde(default)]
    pub profile: Option<ConnectionProfile>,
    #[serde(default)]
    pub routing: RoutingSettings,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoutingSettings {
    #[serde(default)]
    pub default_provider: Option<String>,
    #[serde(default)]
    pub budget_tokens_per_session: Option<u64>,
}

pub fn settings_path(app_config_dir: PathBuf) -> PathBuf {
    app_config_dir.join("settings.json")
}
