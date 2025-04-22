use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use server_side::{Version, APP_DIR};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use zip::ZipArchive;

#[deprecated]
#[get("/desktop-app.jar")]
pub async fn last_app_version() -> Option<NamedFile> {
    let path_file = APP_DIR.to_string() + "/downloads/desktop-app.jar";
    let path_buff = PathBuf::from(path_file);

    NamedFile::open(path_buff).await.ok()
}

#[deprecated]
#[get("/theroundtable-linux-x64.zip")]
pub async fn app_zip() -> Option<NamedFile> {
    let path_file = APP_DIR.to_string() + "/downloads/theroundtable-linux-x64.zip";
    let path_buff = PathBuf::from(path_file);

    NamedFile::open(path_buff).await.ok()
}

#[get("/available?<version>")]
pub async fn available_update(version: String) -> Json<HashMap<String, bool>> {
    let last_version = get_jar_version(APP_DIR.to_string() + "/downloads/desktop-app.jar");

    match last_version {
        Some(last_version) => {
            let response = Version::new(version)
                .cmp(&Version::new(last_version)) == std::cmp::Ordering::Less;
            Json(HashMap::from([
                ("response".to_string(), response),
                ("succesfull".to_string(), true)
            ]))
        }
        None => {
            Json(HashMap::from([
                ("response".to_string(), false),
                ("succesfull".to_string(), false)
            ]))
        }
    }
}

// region: AUXILIARY FUNCTIONS

fn get_jar_version(jar_file_path: String) -> Option<String> {
    extract_version_from_jar(&jar_file_path)
}

fn extract_version_from_jar(jar_file_path: &str) -> Option<String> {
    let jar_file = File::open(jar_file_path).ok()?;
    let mut archive = ZipArchive::new(jar_file).ok()?;

    let entry = archive.by_name("META-INF/maven/org.lebastudios.theroundtable/desktop-app/pom.properties").ok()?;

    let mut properties = String::new();
    let mut reader = BufReader::new(entry);
    reader.read_to_string(&mut properties).ok()?;

    let properties_map: HashMap<String, String> = properties.lines()
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

// endregion: AUXILIARY FUNCTIONS
