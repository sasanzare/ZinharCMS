pub mod config;
pub mod db;
pub mod error;
pub mod middleware;
pub mod models;
pub mod plugins;
pub mod routes;
pub mod services;
pub mod state;

use axum::Router;
use state::AppState;

pub fn app(state: AppState) -> Router {
    routes::router(state)
}
