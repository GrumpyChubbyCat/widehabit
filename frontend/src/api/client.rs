use gloo_net::http::{Request, Response};
use leptos::prelude::*;
use serde::Serialize;
use shared::model::auth::AuthToken;
use web_sys::window;

#[derive(Clone)]
pub struct AuthFlowClient {
    base_url: String,
    token: RwSignal<Option<String>>,
}

// TODO: Implement CBOR support
impl AuthFlowClient {
    pub fn new() -> Self {
        // Get the origin (protocol + host + port)
        let origin = window()
            .map(|w| w.location().origin().unwrap_or_default())
            .unwrap_or_else(|| "http://localhost:8080".to_string());

        // Build the base URL with versioning
        let base_url = format!("{}/api/v1", origin);

        let token = window()
            .and_then(|w| w.local_storage().ok().flatten())
            .and_then(|ls| ls.get_item("wh_auth_token").ok().flatten());

        Self {
            base_url,
            token: RwSignal::new(token),
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.token.get().is_some()
    }

    // Auth methods
    pub async fn register<Req>(&self, path: &str, body: &Req) -> Result<(), String>
    where
        Req: Serialize,
    {
        let url = format!("{}{}", self.base_url, path);

        let resp = Request::post(&url)
            .json(body)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.ok() {
            return Err(format!("Registration failed: {}", resp.status()));
        }

        Ok(())
    }

    /// Public: Login - returns access token and sets it in the client
    pub async fn login<Req>(&self, path: &str, body: &Req) -> Result<(), String>
    where
        Req: Serialize,
    {
        let url = format!("{}{}", self.base_url, path);

        let resp = Request::post(&url)
            .json(body)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.ok() {
            return Err(format!("Login failed: {}", resp.status()));
        }

        let data: AuthToken = resp.json().await.map_err(|e| e.to_string())?;

        // Save token and update state
        self.update_token_state(Some(data.access_token));

        Ok(())
    }

    // Authorized crud operations
    async fn execute<F>(&self, build_fn: F) -> Result<Response, String>
    where
        F: Fn() -> Result<Request, gloo_net::Error>,
    {
        loop {
            // Create a new request instance
            let req = build_fn().map_err(|e| e.to_string())?;

            // Inject authorization headers
            if let Some(token) = self.token.get_untracked() {
                req.headers()
                    .set("Authorization", &format!("Bearer {}", token));
            }

            // Send the request (the 'req' object is consumed here)
            let resp = req.send().await.map_err(|e| e.to_string())?;

            // Handle 401 Unauthorized with a retry logic
            if resp.status() == 401 {
                if self.refresh_token().await.is_ok() {
                    // Retry the loop: build_fn() will generate a fresh Request
                    continue;
                }
            }

            return Ok(resp);
        }
    }

    pub async fn get<Res>(&self, path: &str) -> Result<Res, String>
    where
        Res: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);

        // For GET, call .build() to obtain Result<Request, Error>
        let resp = self.execute(|| Request::get(&url).build()).await?;

        resp.json::<Res>().await.map_err(|e| e.to_string())
    }

    pub async fn post<Req, Res>(&self, path: &str, body: &Req) -> Result<Res, String>
    where
        Req: serde::Serialize,
        Res: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        let body = serde_json::to_value(body).map_err(|e| e.to_string())?;

        // Closure for POST: body is serialized once, but Request is recreated on each retry
        let resp = self.execute(|| Request::post(&url).json(&body)).await?;

        resp.json::<Res>().await.map_err(|e| e.to_string())
    }

    pub async fn patch<Req, Res>(&self, path: &str, body: &Req) -> Result<Res, String>
    where
        Req: serde::Serialize,
        Res: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        let body = serde_json::to_value(body).map_err(|e| e.to_string())?;

        let resp = self.execute(|| Request::patch(&url).json(&body)).await?;

        resp.json::<Res>().await.map_err(|e| e.to_string())
    }

    pub async fn delete<Res>(&self, path: &str) -> Result<Res, String>
    where
        Res: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);

        let resp = self.execute(|| Request::delete(&url).build()).await?;

        // TODO: Implement CBOR support
        // Handle 204 No Content by deserializing "null" into the expected Res type
        if resp.status() == 204 {
            return serde_json::from_str("null").map_err(|e| e.to_string());
        }

        resp.json::<Res>().await.map_err(|e| e.to_string())
    }

    // Helper methods
    pub async fn refresh_token(&self) -> Result<(), ()> {
        let url = format!("{}/auth/refresh", self.base_url);

        // We use a raw Request here instead of self.post to avoid infinite recursion in 'execute'.
        // The browser will automatically include the HttpOnly 'refresh_token' cookie.
        let resp = Request::post(&url).send().await.map_err(|_| ())?;

        if resp.ok() {
            let data: AuthToken = resp.json().await.map_err(|_| ())?;
            self.update_token_state(Some(data.access_token));
            Ok(())
        } else {
            self.logout();
            Err(())
        }
    }

    // Clears the authentication state and removes the token from local storage
    pub fn logout(&self) {
        self.update_token_state(None);
    }

    fn update_token_state(&self, token: Option<String>) {
        self.token.set(token.clone());
        if let Some(ls) = window().and_then(|w| w.local_storage().ok().flatten()) {
            match token {
                Some(t) => {
                    let _ = ls.set_item("wh_auth_token", &t);
                }
                None => {
                    let _ = ls.remove_item("wh_auth_token");
                }
            }
        }
    }
}
