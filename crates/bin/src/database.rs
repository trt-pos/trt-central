use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use std::sync::{LazyLock};

static  DATABASE_URL: LazyLock<String> = LazyLock::new(|| {
    std::env::var("TRT_DB_CONN").expect("TRT_DB_CONN must be set")
});

pub async fn init_pool() -> Pool<MySql> {
    
    
    MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&DATABASE_URL)
        .await
        .expect("Failed to connect to MariaDB")
}
