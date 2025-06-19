#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    SQLxError(#[from] sqlx::Error),
}