use api::{AppState, routes};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE URL must be set!");

    let database = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database!");

    println!("Database connected!");

    let state = AppState {
        app_name: "API".to_string(),
        database: database,
    };

    let app = routes::create_router(state);

    let port: u16 = std::env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse()
        .expect("PORT must be a valid number");

    let address = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    println!("Server running at http://{address}");
    axum::serve(listener, app).await.unwrap();
}
