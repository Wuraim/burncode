use crate::opencode::client::OpencodeClient;
use crate::store::settings::AppSettings;
use crate::store::SettingsStore;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppStateInner {
    pub client: Option<OpencodeClient>,
    pub settings: AppSettings,
    pub settings_store: SettingsStore,
    pub event_handle: Option<tokio::task::AbortHandle>,
}

impl AppStateInner {
    pub fn new(settings_store: SettingsStore) -> Self {
        Self {
            client: None,
            settings: AppSettings::default(),
            settings_store,
            event_handle: None,
        }
    }
}

pub type AppState = Arc<Mutex<AppStateInner>>;
