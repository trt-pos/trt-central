use sqlx::FromRow;
use sqlx::types::chrono::{DateTime, Utc};
use data::Version;
use crate::entities::License;

#[derive(FromRow)]
pub struct AppInstallation {
    uuid: String,
    version: Version,
    license_id: License,
    last_used_at: DateTime<Utc>
}