use std::sync::Arc;
use axum::Extension;
use axum::extract::{Query, State};
use axum::response::Html;
use serde::Deserialize;
use tera::Context;
use crate::router::app::app::{AppState, User};

#[derive(Deserialize)]
pub struct ErrorParams{
    code:u16,
    message:String
}
pub async fn error(
    Query(params):Query<ErrorParams>,
    State(state):State<Arc<AppState>>,
    Extension(current_user):Extension<Option<User>>
) -> Html<String> {
    let mut context = Context::new();
    context.insert("status_code",&params.code);
    context.insert("status_text",&params.message);
    let error = state.tera.render("views/error.html",&context).unwrap();
    let mut context = Context::new();
    context.insert("view",&error);
    context.insert("current_user",&current_user);
    context.insert("with_footer",&true);
    let rendered = state.tera.render("view/main.html",&context).unwrap();
    Html(rendered)
}