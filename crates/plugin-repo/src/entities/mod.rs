use sqlx::{Executor, Sqlite};

mod category;
mod plugin;
mod plugin_version;
mod tag;

pub use category::Category;
pub use plugin::Plugin;
pub use plugin_version::PluginVersion;
pub use tag::Tag;

pub trait Entity {
    async fn insert<'c, E>(&self, executor: E) -> Result<(), crate::Error>
    where
        E: Executor<'c, Database = Sqlite>;
    async fn update<'c, E>(&self, executor: E) -> Result<(), crate::Error>
    where
        E: Executor<'c, Database = Sqlite>;
    async fn delete<'c, E>(&self, executor: E) -> Result<(), crate::Error>
    where
        E: Executor<'c, Database = Sqlite>;
    async fn exists<'c, E>(&self, executor: E) -> Result<bool, crate::Error>
    where
        E: Executor<'c, Database = Sqlite>;
}
