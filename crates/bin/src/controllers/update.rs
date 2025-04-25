use actix_web::{get, web, HttpResponse, Responder};
use data::Version;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use zip::ZipArchive;

#[get("/available?<version>")]
pub async fn available_update(version: web::Path<String>) -> actix_web::Result<impl Responder> {
    let jar_file_path = crate::APP_DIR.to_string() + "/resources/desktop-app.jar";

    let last_version = get_jar_version(&jar_file_path);

    match last_version {
        Some(last_version) => {
            let response = Version::new(version.clone())
                .map_err(actix_web::error::ErrorBadRequest)?
                < Version::new(last_version).expect("has to be valid");

            Ok(HttpResponse::Ok().json(HashMap::from([
                ("response".to_string(), response),
                ("succesfull".to_string(), true),
            ])))
        }
        None => Err(actix_web::error::ErrorInternalServerError(
            "Failed to read version",
        )),
    }
}

fn get_jar_version(jar_file_path: &str) -> Option<String> {
    extract_version_from_jar(jar_file_path)
}

fn extract_version_from_jar(jar_file_path: &str) -> Option<String> {
    let jar_file = File::open(jar_file_path).ok()?;
    let mut archive = ZipArchive::new(jar_file).ok()?;

    let entry = archive
        .by_name("META-INF/maven/org.lebastudios.theroundtable/desktop-app/pom.properties")
        .ok()?;

    let mut properties = String::new();
    let mut reader = BufReader::new(entry);
    reader.read_to_string(&mut properties).ok()?;

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

    properties_map.get("version").cloned()
}
