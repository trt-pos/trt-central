use std::path::PathBuf;
use actix_web::{get, Responder};

#[get("/{resource_name}")]
pub async fn get_resource(resource_name: String) -> actix_web::Result<impl Responder> {
    let path_file = format!("{}/resources", crate::APP_DIR.as_str());
    let path_buff = PathBuf::from(path_file).join(resource_name);

    Ok(actix_files::NamedFile::open(path_buff)?)
}
