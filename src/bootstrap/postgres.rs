use std::sync::Arc;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

use crate::config::postgres::PostgresConfig;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub struct Postgres {
    pool: PgPool,
}

impl Postgres {
    pub fn new(config: &PostgresConfig) -> Result<Arc<Self>, String> {
        let connection_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            config.user, config.password, config.host, config.port, config.database
        );

        let manager = ConnectionManager::<PgConnection>::new(&connection_string);

        let pool = Pool::builder()
            .max_size(10)
            .build(manager)
            .map_err(|e| format!("Failed to create PostgreSQL pool: {}", e))?;

        tracing::info!("PostgreSQL connection pool initialized");

        let postgres = Arc::new(Postgres { pool });

        let mut conn = postgres.conn()?;
        conn.run_pending_migrations(MIGRATIONS)
            .map_err(|e| format!("Failed to run migrations: {}", e))?;
        tracing::info!("Database migrations applied successfully");

        Ok(postgres)
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn conn(&self) -> Result<PgPooledConnection, String> {
        self.pool
            .get()
            .map_err(|e| format!("Failed to get connection from pool: {}", e))
    }
}
