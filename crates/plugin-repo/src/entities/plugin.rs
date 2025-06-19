use crate::entities::Entity;
use data::Version;
use getset::Getters;
use sqlx::{Executor, FromRow, Sqlite, SqlitePool};

#[derive(FromRow, Getters)]
pub struct Plugin {
    #[getset(get = "pub")]
    id: String,
    #[getset(get = "pub")]
    name: String,
    #[getset(get = "pub")]
    last_version: String,
}

impl Plugin {
    pub fn new(id: &str, name: &str, version: &Version) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            last_version: version.to_string(),
        }
    }

    pub async fn get(id: &str, db_pool: &SqlitePool) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(Self, "select id, name, last_version from plugin where id = ?", id)
            .fetch_one(db_pool)
            .await
    }
}

impl Entity for Plugin {
    async fn insert<'c, E>(&self, executor: E) -> Result<(), crate::Error>
    where
        E: Executor<'c, Database = Sqlite>,
    {
        sqlx::query("insert into plugin (id, name, last_version) values (?, ?, ?)")
            .bind(&self.id)
            .bind(&self.name)
            .bind(self.last_version.to_string())
            .execute(executor)
            .await?;
        Ok(())
    }

    async fn update<'c, E>(&self, executor: E) -> Result<(), crate::Error>
    where
        E: Executor<'c, Database = Sqlite>,
    {
        sqlx::query("update plugin set name = ?, last_version = ? where id = ?")
            .bind(&self.name)
            .bind(self.last_version.to_string())
            .bind(&self.id)
            .execute(executor)
            .await?;
        Ok(())
    }

    async fn delete<'c, E>(&self, executor: E) -> Result<(), crate::Error>
    where
        E: Executor<'c, Database = Sqlite>,
    {
        todo!()
    }

    async fn exists<'c, E>(&self, executor: E) -> Result<bool, crate::Error>
    where
        E: Executor<'c, Database = Sqlite>,
    {
        todo!()
    }
}
