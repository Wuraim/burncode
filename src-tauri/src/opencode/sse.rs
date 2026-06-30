use crate::opencode::client::OpencodeClient;
use crate::opencode::types::OpencodeEvent;
use futures::StreamExt;
use reqwest_eventsource::{Event as EventSourceEvent, EventSource};
use serde_json::Value;
use tauri::Emitter;

pub fn start(app_handle: tauri::AppHandle, client: OpencodeClient) -> tokio::task::AbortHandle {
    let handle = tokio::spawn(async move {
        let es = EventSource::new(client.event_request());
        if let Err(e) = &es {
            let _ = app_handle.emit(
                "opencode-event",
                OpencodeEvent {
                    event_type: "sse_error".into(),
                    data: Value::String(e.to_string()),
                },
            );
            return;
        }
        let mut es = es.unwrap();
        loop {
            match es.next().await {
                Some(Ok(EventSourceEvent::Open)) => {
                    let _ = app_handle.emit(
                        "opencode-event",
                        OpencodeEvent {
                            event_type: "server_connected".into(),
                            data: Value::Object(Default::default()),
                        },
                    );
                }
                Some(Ok(EventSourceEvent::Message(msg))) => {
                    let data: Value = serde_json::from_str(&msg.data).unwrap_or_else(|_| {
                        if msg.data.is_empty() {
                            Value::Null
                        } else {
                            Value::String(msg.data.clone())
                        }
                    });
                    let event = OpencodeEvent::from_sse(&msg.event, data);
                    let _ = app_handle.emit("opencode-event", event);
                }
                Some(Err(e)) => {
                    let _ = app_handle.emit(
                        "opencode-event",
                        OpencodeEvent {
                            event_type: "sse_error".into(),
                            data: Value::String(e.to_string()),
                        },
                    );
                    break;
                }
                None => break,
            }
        }
        let _ = es.close();
    });
    handle.abort_handle()
}
