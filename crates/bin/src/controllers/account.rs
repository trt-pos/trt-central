use crate::entities::{Account, License};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::MySqlPool;

#[get("/has_valid_license?<email>")]
pub async fn has_valid_license(
    pool: web::Data<MySqlPool>,
    email: String,
) -> actix_web::Result<impl Responder> {
    let license = License::query_by_email(&email, &pool).await;

    let license = license.ok_or(actix_web::error::ErrorNotFound(format!(
        "License not found for email {}",
        email
    )))?;

    if license.is_valid() {
        Ok(HttpResponse::Ok())
    } else {
        Err(actix_web::error::ErrorUnauthorized(
            "Invalid license or expired",
        ))
    }
}

#[post("/login")]
pub async fn login(
    pool: web::Data<MySqlPool>,
    body: web::Form<LoginBody>,
) -> actix_web::Result<impl Responder> {
    let account = Account::query_by_email(&body.email, &pool).await.ok_or(
        actix_web::error::ErrorUnauthorized("Invalid email or password"),
    )?;

    if account.validate_password(&body.password) {
        Ok(HttpResponse::Ok())
    } else {
        Err(actix_web::error::ErrorUnauthorized(
            "Invalid email or password",
        ))
    }
}

#[get("/validate?<id>")]
pub async fn validate_license(
    pool: web::Data<MySqlPool>,
    id: String,
) -> actix_web::Result<impl Responder> {
    let license = License::query_by_id(&id, &pool)
        .await
        .ok_or(actix_web::error::ErrorNotFound(format!(
            "License is invalid or expired for id {}",
            id
        )))?;

    if license.is_valid() {
        Ok(HttpResponse::Ok())
    } else {
        Err(actix_web::error::ErrorUnauthorized(
            "Invalid license or expired",
        ))
    }
}

#[derive(Deserialize)]
struct LoginBody {
    email: String,
    password: String,
}
