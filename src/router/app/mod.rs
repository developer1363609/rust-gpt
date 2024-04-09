use axum::{
    routing::{get,post},
    Router
};

use std::sync::Arc;
use crate::{middleware::valid_openai_api_key};
use crate::middleware::auth;
use crate::router::app::app::AppState;
use crate::router::app::auth::{form_signup, login, login_form, logout, signup};
use crate::router::app::blog::{blog, blog_by_slug};
use crate::router::app::chat::{chat, chat_add_message, chat_by_id, chat_generate, delete_chat, new_chat};
use crate::router::app::error::error;
use crate::router::app::home::app;
use crate::router::app::settings::{settings, settings_openai_api_key};

mod home;

mod chat;
pub mod app;
mod settings;
mod error;
mod blog;
mod auth;


pub fn app_router(state:Arc<AppState>) -> Router{
    let chat_router = Router::new()
        .route("/", get(chat).post(new_chat))
        .route("/:id", get(chat_by_id).delete(delete_chat))
        .route("/:id/message/add", post(chat_add_message))
        .route("/:id/generate", get(chat_generate))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn(valid_openai_api_key))
        .layer(axum::middleware::from_fn(auth));
    let settings_router = Router::new()
        .route("/",get(settings).post(settings_openai_api_key))
        .layer(axum::middleware::from_fn(auth));
    Router::new()
        .route("/", get(app))
        .route("/error", get(error))
        .route("/login", get(login).post(login_form))
        .route("/signup", get(signup).post(form_signup))
        .route("/logout", get(logout))
        .route("/blog", get(blog))
        .route("/blog/:slug", get(blog_by_slug))
        .nest("/chat", chat_router)
        .nest("/settings", settings_router)
        .with_state(state.clone())
}