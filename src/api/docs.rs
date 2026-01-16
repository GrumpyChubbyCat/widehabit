use crate::api::router::{auth::AUTH_TAG, habit::HABIT_TAG, health::HEALTH_TAG, log::LOG_TAG, schedule::SCHEDULE_TAG};
use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    tags(
        (name = HEALTH_TAG, description = "Service healthckeck"),
        (name = AUTH_TAG, description = "Authorization API endpoints"),
        (name = HABIT_TAG, description = "Habits API endpoints"),
        (name = SCHEDULE_TAG, description = "Habit schedule API endpoints"),
        (name = LOG_TAG, description = "Habit log API endpoints")
    )
)]
pub struct WideApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
            )
        }
    }
}
