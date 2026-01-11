use std::sync::Arc;

use axum::extract::FromRef;

use crate::{config::AuthConfig, service::user::UserService};

#[derive(Clone)]
pub struct AppState {
    pub auth_config: AuthConfig,
    pub user_service: Arc<UserService>,
}

impl AppState {
    pub fn new(auth_config: AuthConfig, user_service: UserService) -> Self {
        let user_service = Arc::new(user_service);
        Self { auth_config, user_service }
    }
}

impl FromRef<AppState> for AuthConfig {
    fn from_ref(state: &AppState) -> Self {
        state.auth_config.clone()
    }
}

// Allows use State(user_service): State<Arc<UserService>>
impl FromRef<AppState> for Arc<UserService> {
    fn from_ref(state: &AppState) -> Self {
        state.user_service.clone()
    }
}