use crate::entities::Entity;
use crate::error::Error;
use data::Version;
use getset::Getters;
use sqlx::{Executor, FromRow, Row, Sqlite};

#[derive(FromRow, Getters)]
pub struct PluginVersion {
    #[getset(get = "pub")]
    id: String,
    #[getset(get = "pub")]
    version: String,
}

impl PluginVersion {
    pub fn new(id: &str, version: &Version) -> Self {
        Self {
            id: id.to_string(),
            version: version.to_string(),
        }
    }
}

impl Entity for PluginVersion {
    async fn insert<'c, E>(&self, executor: E) -> Result<(), Error>
    where
        E: Executor<'c, Database = Sqlite>,
    {
        sqlx::query("insert into plugin_version (plugin_id, version) values (?, ?)")
            .bind(&self.id)
            .bind(&self.version)
            .execute(executor)
            .await?;
        Ok(())
    }

    async fn update<'c, E>(&self, executor: E) -> Result<(), Error>
    where
        E: Executor<'c, Database = Sqlite>,
    {
        todo!()
    }

    async fn delete<'c, E>(&self, executor: E) -> Result<(), Error>
    where
        E: Executor<'c, Database = Sqlite>,
    {
        todo!()
    }

    async fn exists<'c, E>(&self, executor: E) -> Result<bool, Error>
    where
        E: Executor<'c, Database = Sqlite>,
    {
        Ok(
            sqlx::query("select count(*) from plugin_version where plugin_id = ? and version = ?")
                .bind(&self.id)
                .bind(&self.version)
                .fetch_one(executor)
                .await?
                .get::<i64, _>(0)
                > 0,
        )
    }
}
