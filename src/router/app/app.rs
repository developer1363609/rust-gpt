use std::sync::Arc;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{Pool, Sqlite};
use tera::Tera;
use crate::data::repository::ChatRepository;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<Pool<Sqlite>>,
    pub tera: Tera,
    pub chat_repo: ChatRepository,
}

#[derive(Debug, sqlx::FromRow, Serialize, Clone)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub openai_api_key: Option<String>,
}