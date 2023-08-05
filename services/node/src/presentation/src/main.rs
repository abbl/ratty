use application::application::Application;
use axum::Router;
use dotenvy::dotenv;
use state::ApplicationState;
use v1::{
    authorize::state::AuthorizationState,
    register::{registration_routes, registration_state::RegistrationState},
};

pub mod errors;
pub mod extractors;
pub mod middleware;
pub mod state;
pub mod v1;

use crate::v1::authorize::routes as AuthorizationRouter;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let application = Application::new().await;

    let state = ApplicationState {
        authorization_state: AuthorizationState {
            authorization_service: application.authorization_service.clone(),
        },
        registration_state: RegistrationState {
            user_service: application.user_service.clone(),
        },
    };

    let app: Router = Router::new()
        .nest("/v1/authorize", AuthorizationRouter::new())
        .nest("/v1/register", registration_routes::new())
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
