use crate::opencode::types::{ExecuteCommandRequest, HealthResponse, SendPromptRequest};
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use serde_json::Value;

#[derive(Clone)]
pub struct OpencodeClient {
    client: Client,
    base_url: String,
    auth: Option<(String, Option<String>)>,
}

impl OpencodeClient {
    pub fn new(base_url: &str, username: &str, password: &str) -> Self {
        let auth = if username.is_empty() && password.is_empty() {
            None
        } else {
            Some((username.to_string(), Some(password.to_string())))
        };
        let base_url = base_url.trim_end_matches('/').to_string();
        Self {
            client: Client::new(),
            base_url,
            auth,
        }
    }

    fn auth(&self, req: RequestBuilder) -> RequestBuilder {
        if let Some((user, pass)) = &self.auth {
            req.basic_auth(user, pass.clone())
        } else {
            req
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    pub async fn health(&self) -> Result<HealthResponse, String> {
        self.get("/global/health").await
    }

    pub async fn get_projects(&self) -> Result<Value, String> {
        self.get("/project").await
    }

    pub async fn get_current_project(&self) -> Result<Value, String> {
        self.get("/project/current").await
    }

    pub async fn get_providers(&self) -> Result<Value, String> {
        self.get("/provider").await
    }

    pub async fn get_provider_auth_methods(&self) -> Result<Value, String> {
        self.get("/provider/auth").await
    }

    pub async fn get_config_providers(&self) -> Result<Value, String> {
        self.get("/config/providers").await
    }

    pub async fn get_agents(&self) -> Result<Value, String> {
        self.get("/agent").await
    }

    pub async fn get_sessions(&self) -> Result<Value, String> {
        self.get("/session").await
    }

    pub async fn create_session(&self, title: Option<String>) -> Result<Value, String> {
        let mut body = serde_json::Map::new();
        if let Some(t) = title {
            body.insert("title".to_string(), Value::String(t));
        }
        self.post("/session", Value::Object(body)).await
    }

    pub async fn get_session(&self, id: &str) -> Result<Value, String> {
        self.get(&format!("/session/{}", id)).await
    }

    pub async fn get_messages(&self, id: &str) -> Result<Value, String> {
        self.get(&format!("/session/{}/message", id)).await
    }

    pub async fn send_prompt(&self, id: &str, request: SendPromptRequest) -> Result<Value, String> {
        self.post(&format!("/session/{}/message", id), serde_json::to_value(request).unwrap())
            .await
    }

    pub async fn get_todos(&self, id: &str) -> Result<Value, String> {
        self.get(&format!("/session/{}/todo", id)).await
    }

    pub async fn get_diff(&self, id: &str) -> Result<Value, String> {
        self.get(&format!("/session/{}/diff", id)).await
    }

    pub async fn set_provider_auth(&self, provider_id: &str, body: Value) -> Result<bool, String> {
        let res: Value = self.put(&format!("/auth/{}", provider_id), body).await?;
        Ok(res.as_bool().unwrap_or(false))
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, String> {
        let req = self.client.get(self.url(path));
        let res = self.auth(req).send().await.map_err(|e| e.to_string())?;
        let status = res.status();
        if !status.is_success() {
            let text = res.text().await.unwrap_or_default();
            return Err(format!("{}: {}", status, text));
        }
        res.json::<T>().await.map_err(|e| e.to_string())
    }

    pub async fn post<T: DeserializeOwned>(&self, path: &str, body: Value) -> Result<T, String> {
        let req = self.client.post(self.url(path)).json(&body);
        let res = self.auth(req).send().await.map_err(|e| e.to_string())?;
        let status = res.status();
        if !status.is_success() {
            let text = res.text().await.unwrap_or_default();
            return Err(format!("{}: {}", status, text));
        }
        res.json::<T>().await.map_err(|e| e.to_string())
    }

    pub async fn patch<T: DeserializeOwned>(&self, path: &str, body: Value) -> Result<T, String> {
        let req = self.client.patch(self.url(path)).json(&body);
        let res = self.auth(req).send().await.map_err(|e| e.to_string())?;
        let status = res.status();
        if !status.is_success() {
            let text = res.text().await.unwrap_or_default();
            return Err(format!("{}: {}", status, text));
        }
        res.json::<T>().await.map_err(|e| e.to_string())
    }

    pub async fn put<T: DeserializeOwned>(&self, path: &str, body: Value) -> Result<T, String> {
        let req = self.client.put(self.url(path)).json(&body);
        let res = self.auth(req).send().await.map_err(|e| e.to_string())?;
        let status = res.status();
        if !status.is_success() {
            let text = res.text().await.unwrap_or_default();
            return Err(format!("{}: {}", status, text));
        }
        res.json::<T>().await.map_err(|e| e.to_string())
    }

    pub async fn update_session(&self, id: &str, title: Option<String>) -> Result<Value, String> {
        let mut body = serde_json::Map::new();
        if let Some(t) = title {
            body.insert("title".to_string(), Value::String(t));
        }
        self.patch(
            &format!("/session/{}", id),
            Value::Object(body),
        ).await
    }

    pub async fn delete_session(&self, id: &str) -> Result<bool, String> {
        let req = self.client.delete(self.url(&format!("/session/{}", id)));
        let res = self.auth(req).send().await.map_err(|e| e.to_string())?;
        let status = res.status();
        if !status.is_success() {
            let text = res.text().await.unwrap_or_default();
            return Err(format!("{}: {}", status, text));
        }
        Ok(res.json::<Value>().await.map_err(|e| e.to_string())?.as_bool().unwrap_or(false))
    }

    pub async fn respond_to_permission(
        &self,
        session_id: &str,
        permission_id: &str,
        response: &str,
        remember: Option<bool>,
    ) -> Result<bool, String> {
        let mut body = serde_json::Map::new();
        body.insert("response".to_string(), Value::String(response.to_string()));
        if let Some(r) = remember {
            body.insert("remember".to_string(), Value::Bool(r));
        }
        let res: Value = self.post(
            &format!("/session/{}/permissions/{}", session_id, permission_id),
            Value::Object(body),
        ).await?;
        Ok(res.as_bool().unwrap_or(false))
    }

    pub async fn fork_session(&self, id: &str, message_id: Option<&str>) -> Result<Value, String> {
        let mut body = serde_json::Map::new();
        if let Some(mid) = message_id {
            body.insert("messageID".to_string(), Value::String(mid.to_string()));
        }
        self.post(
            &format!("/session/{}/fork", id),
            Value::Object(body),
        ).await
    }

    pub async fn get_commands(&self) -> Result<Value, String> {
        self.get("/command").await
    }

    pub async fn execute_command(&self, id: &str, request: ExecuteCommandRequest) -> Result<Value, String> {
        self.post(
            &format!("/session/{}/command", id),
            serde_json::to_value(request).unwrap(),
        ).await
    }

    pub async fn abort_session(&self, id: &str) -> Result<bool, String> {
        let res: Value = self.post(
            &format!("/session/{}/abort", id),
            serde_json::Value::Null,
        ).await?;
        Ok(res.as_bool().unwrap_or(false))
    }

    pub async fn prompt_async(&self, id: &str, request: SendPromptRequest) -> Result<(), String> {
        self.post_empty(
            &format!("/session/{}/prompt_async", id),
            serde_json::to_value(request).unwrap(),
        )
        .await
    }

    pub async fn post_empty(&self, path: &str, body: Value) -> Result<(), String> {
        let req = self.client.post(self.url(path)).json(&body);
        let res = self.auth(req).send().await.map_err(|e| e.to_string())?;
        let status = res.status();
        if !status.is_success() {
            let text = res.text().await.unwrap_or_default();
            return Err(format!("{}: {}", status, text));
        }
        Ok(())
    }

    pub fn event_request(&self) -> RequestBuilder {
        let req = self.client.get(self.url("/event"));
        self.auth(req)
    }
}
