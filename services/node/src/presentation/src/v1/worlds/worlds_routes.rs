use axum::{extract::State, http::StatusCode, routing::post, Router};
use garde::Validate;
use serde::Deserialize;

use crate::extractors::require_authorization::RequireAuthorization;
use crate::extractors::validated_json::ValidatedJson;
use crate::state::application_state::ApplicationState;

use super::worlds_state::WorldsState;

#[derive(Deserialize, Validate)]
struct CreateWorldDto {
    #[garde(length(min = 3))]
    name: String,

    #[garde(length(min = 1))]
    description: Option<String>,
}

pub fn new() -> Router<ApplicationState> {
    Router::new().route("/", post(create_world))
}

async fn create_world(
    RequireAuthorization(user, _): RequireAuthorization<WorldsState>,
    ValidatedJson(payload): ValidatedJson<CreateWorldDto>,
) -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}
