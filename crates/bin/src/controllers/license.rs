use rocket::serde::json::Json;
use rocket::State;
use server_side::entities::License;
use sqlx::MySqlPool;
use std::collections::HashMap;

#[get("/validate?<license_id>")]
pub async fn validate_license(
    pool: &State<MySqlPool>,
    license_id: &str,
) -> Json<HashMap<String, bool>> {
    let license = match sqlx::query_as::<_, License>("SELECT * FROM License WHERE id = ?")
        .bind(license_id)
        .fetch_one(pool.inner())
        .await
    {
        Ok(value) => value,
        Err(_) => return Json(HashMap::from([("message".to_string(), false)])),
    };

    Json(HashMap::from([("message".to_string(), license.is_valid())]))
}
