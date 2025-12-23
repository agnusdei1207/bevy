//! Legend API Server - Pure Axum REST API
//!
//! Build: cargo build --features server
//! Run: cargo run --features server

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use axum::{
        routing::{get, post},
        Router,
    };
    use tower_http::cors::{CorsLayer, Any};
    use tower_http::services::ServeDir;
    use tracing_subscriber;
    
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://legend:legend@db:5432/legend".to_string());
    
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");
    
    // Initialize global DB pool
    legend_client::server::db::init_db(pool.clone());
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    
    println!("✅ Database connected and migrated");
    
    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    // API Router
    let api_routes = Router::new()
        .route("/health", get(health_check))
        .route("/login", post(legend_client::server::auth::login_handler))
        .route("/register", post(legend_client::server::auth::register_handler))
        .route("/monsters", get(legend_client::server::monsters::get_monsters))
        .route("/monsters/{id}", get(legend_client::server::monsters::get_monster_by_id))
        .route("/skills", get(legend_client::server::skills::get_skills));
    
    // Main Router
    let app = Router::new()
        .nest("/api", api_routes)
        .nest_service("/assets", ServeDir::new("public/assets"))
        .layer(cors)
        .layer(axum::Extension(pool));
    
    let addr = "0.0.0.0:3000";
    println!("🎮 Legend API Server: http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(feature = "server")]
async fn health_check() -> &'static str {
    "OK"
}

#[cfg(not(feature = "server"))]
pub fn main() {
    // This binary requires --features server
    println!("Run with: cargo run --features server");
}
