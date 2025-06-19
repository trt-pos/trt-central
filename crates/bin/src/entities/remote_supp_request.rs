use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;

pub enum Status {
    Pending,
    Accepted,
    Rejected,
    Cancelled,
    Finished,
}

#[derive(FromRow)]
pub struct RemoteSuppRequest {
    id: u32,
    uuid: String, 
    created_at: DateTime<Utc>,
    status: Status 
}