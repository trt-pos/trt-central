use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Deserialize, Serialize)]
pub struct Account {
    id: i32,
    password: String,
    email: String,
}

impl Account {
    pub async fn query_by_email(email: &str, pool: &sqlx::MySqlPool) -> Option<Self> {
        sqlx::query_as::<_, Account>("SELECT * FROM Account WHERE email = ?")
            .bind(email)
            .fetch_one(pool)
            .await
            .ok()
    }
}

impl Account {
    pub fn validate_password(&self, password: &str) -> bool {
        self.password == password
    }
}