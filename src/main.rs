use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use axum::http::StatusCode;
use axum::Router;
use sqlx::migrate::Migrator;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tera::Tera;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use data::repository::ChatRepository;
use rust_gpt::{data, middleware};
use rust_gpt::router::app::app::AppState;
use rust_gpt::router::app::app_router;

#[tokio::main]
#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let db_path = dotenv::var("DATABASE_PATH").unwrap();
    let options = SqliteConnectOptions::new()
        .filename(db_path)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect_with(options)
        .await.expect("can't connect to database");
    let migrator = Migrator::new(Path::new(dotenv::var("MIGRATIONS_PATH").unwrap().as_str()))
        .await.unwrap();
    migrator.run(&pool).await.unwrap();
    let pool = Arc::new(pool);
    let chat_repo = ChatRepository{
        pool:pool.clone()
    };
    let static_files = ServeDir::new("assets");
    let tera = match Tera::new("templates/**/*"){
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let state = AppState{
        pool,
        tera,
        chat_repo
    };
    let shared_app_state = Arc::new(state);
    let app = Router::new()
        .nest_service("/assets", static_files)
        .with_state(shared_app_state.clone())
        .nest("/", app_router(shared_app_state.clone()))
        .layer(axum::middleware::from_fn_with_state(
            shared_app_state.clone(),
            middleware::handle_error,
        ))
        .layer(axum::middleware::from_fn_with_state(
            shared_app_state.clone(),
            middleware::extract_user,
        ))
        .layer(CookieManagerLayer::new());

    // run it with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

