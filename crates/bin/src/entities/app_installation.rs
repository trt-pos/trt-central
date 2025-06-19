use crate::entities::License;
use data::Version;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow)]
pub struct AppInstallation {
    uuid: String,
    version: Version,
    license_id: License,
    last_used_at: DateTime<Utc>
}