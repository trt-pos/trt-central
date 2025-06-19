use crate::entities::Entity;
use crate::error::Error;
use sqlx::{Executor, FromRow, Row, Sqlite, SqlitePool};

#[derive(FromRow)]
pub struct Category {
    name: String,
}

impl Category {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Entity for Category {
    async fn insert<'c, E>(&self, executor: E) -> Result<(), Error>
    where
        E: Executor<'c, Database = Sqlite>
    {
        sqlx::query("insert into category (name) values (?)")
            .bind(&self.name)
            .execute(executor)
            .await?;
        Ok(())
    }

    async fn update<'c, E>(&self, executor: E) -> Result<(), Error>
    where
        E: Executor<'c, Database = Sqlite>
    {
        todo!()
    }

    async fn delete<'c, E>(&self, executor: E) -> Result<(), Error>
    where
        E: Executor<'c, Database = Sqlite>
    {
        todo!()
    }

    async fn exists<'c, E>(&self, executor: E) -> Result<bool, Error>
    where
        E: Executor<'c, Database = Sqlite>
    {
        Ok(
            sqlx::query("select count(*) from category where name = ?")
                .bind(&self.name)
                .fetch_one(executor)
                .await?
                .get::<i64, _>(0) > 0,
        )
    }
}