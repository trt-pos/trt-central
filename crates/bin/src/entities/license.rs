use serde::ser::SerializeMap;
use serde::Serialize;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow)]
pub struct License {
    id: String,
    max_devices: u32,
    end_date: DateTime<Utc>,
}

impl License {
    pub async fn query_by_email(email: &str, pool: &sqlx::MySqlPool) -> Option<Self> {
        sqlx::query_as::<_, License>(
            "SELECT * FROM License WHERE owner_account_id in (SELECT id FROM Account WHERE email = ?)",
        )
            .bind(email)
            .fetch_one(pool)
            .await
            .ok()
    }
    
    pub async fn query_by_id(id: &str, pool: &sqlx::MySqlPool) -> Option<Self> {
        sqlx::query_as::<_, License>("SELECT * FROM License WHERE id = ?")
            .bind(id)
            .fetch_one(pool)
            .await
            .ok()
    }
    
}

impl License {
    pub fn is_valid(&self) -> bool {
        self.end_date > Utc::now()
    }
}

impl Serialize for License {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("end_date", &self.end_date.to_string())?;
        map.end()
    }
}
