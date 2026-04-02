use std::time::Duration;
use sqlx::pool::PoolOptions;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

#[derive(Debug, Clone)]
pub struct DBService {
    pub connection: Pool<Postgres>,
}


impl DBService {
    pub async fn new() -> DBService {
        let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new()
        .max_connections(10)
            .acquire_timeout(Duration::from_secs(5))
        .connect(&url)
        .await
        .expect("Failed to connect to the database") ;

        DBService {
            connection: pool
        }
    }

    pub async fn create_user(&self) {
        sqlx::query("select 1").fetch_one(&self.connection).await.unwrap();
    }
}