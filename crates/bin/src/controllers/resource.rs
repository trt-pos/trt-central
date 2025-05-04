use actix_web::{get, web, Responder};
use std::path::PathBuf;

#[get("/{resource_name}")]
pub async fn get_resource(resource_name: web::Path<String>) -> actix_web::Result<impl Responder> {
    let resource_path = PathBuf::from(&*crate::APP_DIR)
        .join("resources")
        .join(&*resource_name);

    Ok(actix_files::NamedFile::open(resource_path)?)
}
