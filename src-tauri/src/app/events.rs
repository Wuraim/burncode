use crate::app::state::AppState;
use crate::opencode::sse;

pub fn start_event_stream(app_handle: tauri::AppHandle, state: AppState) {
    let app_handle_clone = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        let client = {
            let guard = state.lock().await;
            guard.client.clone()
        };
        if let Some(client) = client {
            let handle = sse::start(app_handle_clone, client);
            state.lock().await.event_handle = Some(handle);
        }
    });
}

pub fn stop_event_stream(state: AppState) {
    tauri::async_runtime::spawn(async move {
        if let Some(handle) = state.lock().await.event_handle.take() {
            handle.abort();
        }
    });
}
