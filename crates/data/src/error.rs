#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid string format: {0}")]
    InvalidStringFormat(&'static str),
}