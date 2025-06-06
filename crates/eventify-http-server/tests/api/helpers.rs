use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

use eventify_configs::configs::{ApplicationConfig, DatabaseConfig, ServerConfig};

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub db_pool: PgPool,
}

/// Run the server as a background task
pub async fn spawn_app() -> TestApp {
    //Lazy::force(&TRACING);

    // Randomise configuration to ensure test isolation
    let configuration = {
        //let mut c = get_configuration().expect("Failed to read configuration.");
        let mut c = ApplicationConfig {
            server: ServerConfig {
                host: String::from("localhost"),
                port: 0,
            },
            database: DatabaseConfig {
                host: String::from("localhost"),
                port: 5432,
                username: String::from("postgres"),
                password: String::from("password"),
                database_name: String::from(""),
                require_ssl: false,
            },
        };

        // Use a different database for each test case
        c.database.database_name = Uuid::new_v4().to_string();
        // Use a random OS port
        c.server.port = 0;

        c
    };

    // Create and migrate the database
    let pool = configure_database(&configuration.database).await;

    // Launch the application as a background task
    let application =
        eventify_http_server::startup::Application::build(configuration.clone(), pool.clone())
            .await
            .expect("Failed to build application.");
    let application_port = application.port();
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address: format!("http://localhost:{}", application_port),
        port: application_port,
        db_pool: pool,
    }
}

async fn configure_database(config: &DatabaseConfig) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("../../migrations/")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
