use rocket::data::ToByteUnit;
use rocket::serde::json::Json;
use rocket::tokio::io::AsyncReadExt;
use rocket::{Data, State};
use serde::Deserialize;
use server_side::entities::{Account, License};
use sqlx::MySqlPool;
use std::collections::HashMap;

#[get("/has_valid_license", data = "<account>")]
pub async fn has_valid_license<'a>(
    pool: &State<MySqlPool>,
    account: Data<'a>,
) -> Result<Json<HashMap<String, bool>>, &'a str> {
    let account = LoggingBody::from_request_data(account).await?;

    let license = match sqlx::query_as::<_, License>("SELECT * FROM License WHERE owner_account_id in (SELECT id FROM Account WHERE email = ? AND password = ?)")
        .bind(account.email)
        .bind(account.password)
        .fetch_one(pool.inner())
        .await
    {
        Ok(value) => value,
        Err(_) => return Ok(Json(HashMap::from([("message".to_string(), false)]))),
    };

    if !license.is_valid() {
        return Ok(Json(HashMap::from([("message".to_string(), false)])));
    }

    Ok(Json(HashMap::from([("message".to_string(), true)])))
}

#[get("/login", data = "<account>")]
pub async fn login<'a>(
    pool: &State<MySqlPool>,
    account: Data<'a>,
) -> Result<Json<HashMap<String, bool>>, &'a str> {
    let account: LoggingBody = LoggingBody::from_request_data(account).await?;

    match sqlx::query_as::<_, Account>("SELECT * FROM Account WHERE email = ? AND password = ?")
        .bind(account.email)
        .bind(account.password)
        .fetch_one(pool.inner())
        .await
    {
        Ok(_) => Ok(Json(HashMap::from([("message".to_string(), true)]))),
        Err(_) => Err("Failed to fetch account"),
    }
}

// region: Bodies

#[derive(Deserialize)]
struct LoggingBody {
    email: String,
    password: String,
}

impl LoggingBody {
    async fn from_request_data(data: Data<'_>) -> Result<Self, &str> {
        let mut body = String::new();
        if (data.open(128.kilobytes()).read_to_string(&mut body).await).is_err() {
            return Err("Failed to read request body");
        }

        match serde_json::from_str(&body) {
            Ok(account) => Ok(account),
            Err(_) => Err("Failed to parse request body"),
        }
    }
}

// endregion: Bodies
