use std::sync::Arc;

use axum::extract::FromRef;

use crate::{
    config::AuthConfig,
    service::{habit::HabitService, schedule::HabitScheduleService, user::UserService},
};

#[derive(Clone)]
pub struct AppState {
    pub auth_config: AuthConfig,
    pub user_service: Arc<UserService>,
    pub habit_service: Arc<HabitService>,
    pub schedule_service: Arc<HabitScheduleService>,
}

impl AppState {
    pub fn new(
        auth_config: AuthConfig,
        user_service: UserService,
        habit_service: HabitService,
        schedule_service: HabitScheduleService,
    ) -> Self {
        let user_service = Arc::new(user_service);
        let habit_service = Arc::new(habit_service);
        let schedule_service = Arc::new(schedule_service);

        Self {
            auth_config,
            user_service,
            habit_service,
            schedule_service,
        }
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

impl FromRef<AppState> for Arc<HabitService> {
    fn from_ref(state: &AppState) -> Self {
        state.habit_service.clone()
    }
}

impl FromRef<AppState> for Arc<HabitScheduleService> {
    fn from_ref(state: &AppState) -> Self {
        state.schedule_service.clone()
    }
}
