use std::path::PathBuf;
use rocket::fs::NamedFile;
use server_side::APP_DIR;

pub mod account;
pub mod plugin;
pub mod update;
pub mod license;
pub mod download;

#[deprecated]
#[get("/installer.sh")]
pub async fn app_installer() -> Option<NamedFile> {
    let path_file = APP_DIR.to_string() + "/downloads/installer.sh";
    let path_buff = PathBuf::from(path_file);

    NamedFile::open(path_buff).await.ok()
}