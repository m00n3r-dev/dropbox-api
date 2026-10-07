use api::{AppState, routes};
use axum::http::{HeaderValue, Method};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // === logging ===
    let file_appender = tracing_appender::rolling::daily("logs", "app.log");

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    fmt()
        .with_ansi(false)
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_writer(non_blocking)
        .init();

    tracing::info!("Server Starting!");

    // === database ===
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE URL must be set!");
    let database = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database!");

    tracing::info!("Database connected!");

    // === app state ===
    let state = AppState {
        app_name: "API".to_string(),
        database: database,
    };

    // === server ===
    let mut app = routes::create_router(state);

    // === cors setup ===

    let frontend_url = std::env::var("FRONTEND_URL").expect("FRONTEND_URL must be set");
    let cors = CorsLayer::new()
        .allow_origin(frontend_url.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ]);
    app = app.layer(cors);

    let port: u16 = std::env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse()
        .expect("PORT must be a valid number");

    let address = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    tracing::info!("Server running at http://{address}");
    axum::serve(listener, app).await.unwrap();
}
