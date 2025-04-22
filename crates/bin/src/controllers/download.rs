use std::path::PathBuf;
use rocket::fs::NamedFile;
use server_side::APP_DIR;

#[get("/<resource_name>")]
pub async fn get_resource(resource_name: &str) -> Option<NamedFile> {
    let path_file = APP_DIR.to_string() + "/downloads";
    let path_buff = PathBuf::from(path_file).join(resource_name);

    NamedFile::open(path_buff).await.ok()
}