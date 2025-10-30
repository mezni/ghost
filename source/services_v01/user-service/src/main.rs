use std::sync::Arc;
use tracing::info;
use user_service::{config::Config, db::Database, handlers, telemetry};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize telemetry
    telemetry::init_telemetry("user-service")?;

    // Load configuration
    let config = Config::from_env()?;
    info!("Loaded configuration: {:?}", config);

    // Initialize database
    let db = Database::connect(&config.database_url).await?;
    info!("Database connected successfully");

    // Run migrations
    db.run_migrations().await?;
    info!("Database migrations completed");

    // Initialize services
    let user_service = user_service::services::UserService::new(
        db.user_repository(),
        db.keycloak_client(&config).await?,
    );

    // Start HTTP server
    let routes = handlers::create_routes(user_service);
    info!("User Service starting on :{}", config.port);

    warp::serve(routes).run(([0, 0, 0, 0], config.port)).await;

    Ok(())
}
