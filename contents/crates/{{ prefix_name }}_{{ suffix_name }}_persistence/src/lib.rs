pub mod settings;

use anyhow::Result;
use sqlx::MySqlPool;
use settings::PersistenceSettings;

pub use sqlx::MySqlPool as DbPool;

#[derive(Clone, Debug)]
pub struct PersistencePool {
    pool: MySqlPool,
}

impl PersistencePool {
    pub async fn connect(settings: &PersistenceSettings) -> Result<Self> {
        let pool = sqlx::mysql::MySqlPoolOptions::new()
            .max_connections(settings.max_connections.unwrap_or(5))
            .connect(&settings.url)
            .await?;

        if settings.run_migrations.unwrap_or(true) {
            tracing::info!("Running database migrations...");
            sqlx::migrate!("./migrations").run(&pool).await?;
        }

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &MySqlPool {
        &self.pool
    }

    pub async fn migrate_up(&self) -> Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }

    pub async fn migrate_down(&self, steps: Option<u32>) -> Result<()> {
        let migrator = sqlx::migrate!("./migrations");
        let mut conn = self.pool.acquire().await?;
        for _ in 0..steps.unwrap_or(1) {
            migrator.undo(&mut *conn, 0).await?;
        }
        Ok(())
    }
}
