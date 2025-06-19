use std::collections::HashMap;
use std::ops::Deref;
use actix_web::{get, web, Responder};
use sqlx::{Row, SqlitePool};
use tokio::sync::Mutex;

#[get("")]
pub async fn get_all(
    db_pool: web::Data<Mutex<SqlitePool>>,
) -> actix_web::Result<impl Responder> {
    let guard = db_pool.lock().await;
    let categories: Vec<String> = sqlx::query("select name from category")
        .fetch_all(guard.deref())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
        .iter().map(|r| r.get(0))
        .collect();
    
    let mut response = HashMap::new();
    response.insert("categories", categories);
    
    Ok(web::Json(response))
}