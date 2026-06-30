mod app;
mod commands;
mod opencode;
mod router;
mod store;
mod telemetry;

use crate::app::events::start_event_stream;
use crate::app::state::{AppState, AppStateInner};
use crate::commands::connection::{connect, disconnect, is_connected, load_settings, save_settings};
use crate::commands::projects::{current_project, list_projects};
use crate::commands::providers::{
    list_config_providers, list_provider_auth_methods, list_providers, set_provider_auth,
};
use crate::commands::sessions::{
    create_session, get_diff, get_session, get_session_messages, get_todos, list_sessions,
    send_prompt, suggest_route,
};
use crate::commands::telemetry::get_telemetry;
use crate::opencode::client::OpencodeClient;
use crate::store::SettingsStore;
use std::sync::Arc;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
            let settings_store = SettingsStore::new(config_dir);
            let state: AppState = Arc::new(tokio::sync::Mutex::new(AppStateInner::new(
                settings_store.clone(),
            )));
            app.manage(state.clone());

            let handle = app.handle().clone();
            let state_for_setup = state.clone();
            tauri::async_runtime::spawn(async move {
                if let Ok(settings) = settings_store.load().await {
                    let auto_connect = settings
                        .profile
                        .as_ref()
                        .map(|p| p.auto_connect)
                        .unwrap_or(false);
                    {
                        let mut guard = state_for_setup.lock().await;
                        guard.settings = settings;
                    }
                    if auto_connect {
                        let maybe_client = {
                            let guard = state_for_setup.lock().await;
                            guard.settings.profile.clone().and_then(|profile| {
                                Some(OpencodeClient::new(
                                    &profile.server_url,
                                    &profile.username,
                                    &profile.password,
                                ))
                            })
                        };
                        if let Some(client) = maybe_client {
                            if client.health().await.is_ok() {
                                state_for_setup.lock().await.client = Some(client);
                                start_event_stream(handle, state_for_setup);
                            }
                        }
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_settings,
            save_settings,
            connect,
            disconnect,
            is_connected,
            list_projects,
            current_project,
            list_providers,
            list_provider_auth_methods,
            list_config_providers,
            set_provider_auth,
            list_sessions,
            create_session,
            get_session,
            get_session_messages,
            send_prompt,
            suggest_route,
            get_todos,
            get_diff,
            get_telemetry,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
