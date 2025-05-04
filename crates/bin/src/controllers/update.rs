use actix_web::{get, web, HttpResponse, Responder};
use data::Version;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use zip::ZipArchive;

#[derive(Deserialize)]
pub struct AvailableVersionQuery {
    version: String,
}

#[get("/available")]
pub async fn available_update(
    query: web::Query<AvailableVersionQuery>,
) -> actix_web::Result<impl Responder> {
    let jar_file_path = crate::APP_DIR.to_string() + "/resources/desktop-app.jar";

    let last_version = get_jar_version(&jar_file_path)?;

    let response = Version::new(query.version.clone())
        .map_err(actix_web::error::ErrorBadRequest)?
        < Version::new(last_version).expect("has to be valid");

    Ok(HttpResponse::Ok().json(HashMap::from([
        ("response".to_string(), response),
    ])))
}

fn get_jar_version(jar_file_path: &str) -> Result<String, actix_web::Error> {
    extract_version_from_jar(jar_file_path)
}

fn extract_version_from_jar(jar_file_path: &str) -> Result<String, actix_web::Error> {
    let jar_file = File::open(jar_file_path)?;
    let mut archive = ZipArchive::new(jar_file)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let entry = archive
        .by_name("META-INF/maven/org.lebastudios.theroundtable/desktop-app/pom.properties")
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let mut properties = String::new();
    let mut reader = BufReader::new(entry);
    reader.read_to_string(&mut properties)?;

    let properties_map: HashMap<String, String> = properties
        .lines()
        .filter_map(|line| {
            let mut split = line.splitn(2, '=');
            if let (Some(key), Some(value)) = (split.next(), split.next()) {
                Some((key.trim().to_string(), value.trim().to_string()))
            } else {
                None
            }
        })
        .collect();

    match properties_map.get("version") {
        None => Err(actix_web::error::ErrorInternalServerError(
            "version property not found",
        )),
        Some(v) => Ok(v.clone()),
    }
}
