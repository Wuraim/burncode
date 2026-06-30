use crate::opencode::client::OpencodeClient;
use crate::store::routing_analytics::{RoutingAnalytics, RoutingAnalyticsStore};
use crate::store::settings::AppSettings;
use crate::store::SettingsStore;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppStateInner {
    pub client: Option<OpencodeClient>,
    pub settings: AppSettings,
    pub settings_store: SettingsStore,
    pub analytics: RoutingAnalytics,
    pub analytics_store: RoutingAnalyticsStore,
    pub event_handle: Option<tokio::task::AbortHandle>,
}

impl AppStateInner {
    pub fn new(
        settings_store: SettingsStore,
        analytics_store: RoutingAnalyticsStore,
        analytics: RoutingAnalytics,
    ) -> Self {
        Self {
            client: None,
            settings: AppSettings::default(),
            settings_store,
            analytics,
            analytics_store,
            event_handle: None,
        }
    }
}

pub type AppState = Arc<Mutex<AppStateInner>>;
