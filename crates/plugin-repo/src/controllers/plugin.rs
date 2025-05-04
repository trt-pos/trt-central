use crate::data::*;
use actix_web::{get, post, put, web, Responder};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Cursor, Read, Write};
use std::path::PathBuf;
use std::sync::LazyLock;
use std::{fs, io};

#[derive(Deserialize)]
struct ResourceIdentifier {
    plugin_id: String,
    version: String,
    resource: String,
}

#[get("/{plugin_id}/{version}/{resource}")]
pub async fn get_plugin_resource(
    path: web::Path<ResourceIdentifier>,
) -> actix_web::Result<impl Responder> {
    get_plugin_res(&path).await
}

async fn get_plugin_res(
    resource_identifier: &ResourceIdentifier,
) -> actix_web::Result<impl Responder + use<>> {
    let plugin = Plugin::new(&resource_identifier.plugin_id, &resource_identifier.version)?;
    let resource = PluginResource::try_from(resource_identifier.resource.as_str())?;
    
    let path_buff = PathBuf::from(plugin.get_plugin_resource(&resource));

    if !path_buff.exists() {
        return Err(actix_web::error::ErrorNotFound(format!(
            "plugin {} not found in the repository",
            plugin
        )));
    }

    Ok(actix_files::NamedFile::open(path_buff))
}

#[post("/")]
pub async fn post_plugin(
    body: web::Bytes,
    _auth: crate::middleware::PluginPublishingAuthToken,
) -> actix_web::Result<impl Responder> {
    persist_plugin(body, |plugin| {
        let path_buff = PathBuf::from(plugin.get_resource_path());

        if path_buff.exists() {
            return Err(actix_web::error::ErrorConflict(format!(
                "plugin {} already exists in the repository",
                plugin
            )));
        }

        Ok(())
    })
    .await
}

#[put("/")]
pub async fn put_plugin(
    body: web::Bytes,
    _auth: crate::middleware::PluginPublishingAuthToken,
) -> actix_web::Result<impl Responder> {
    persist_plugin(body, |plugin| {
        let plugin_version_dir = PathBuf::from(plugin.get_resource_path());

        if !plugin_version_dir.exists() {
            return Err(actix_web::error::ErrorNotFound(format!(
                "plugin {} doesn't exist in the repository",
                plugin
            )));
        }

        Ok(())
    })
    .await
}

async fn persist_plugin<F>(body: web::Bytes, is_valid: F) -> actix_web::Result<impl Responder>
where
    F: Fn(&Plugin) -> Result<(), actix_web::error::Error>,
{
    let plugin = Plugin::try_from(body.as_ref())?;

    is_valid(&plugin)?;

    let temp_dir = std::env::temp_dir().join(format!("plugin_tmp_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir)?;

    let resource_dir = PathBuf::from(plugin.get_resource_path());
    extract_and_save_plugin_from_zip(&temp_dir, body, &resource_dir).await?;

    let _ = fs::remove_dir(temp_dir);

    // Updating the plugin repo metadata
    let mut metadata = Metadata::load(plugin.plugin_id())?;
    metadata.add_version(plugin.version().clone());
    metadata.save()?;

    Ok(actix_web::HttpResponse::Ok())
}

async fn extract_and_save_plugin_from_zip(
    temp_dir: &PathBuf,
    jar: web::Bytes,
    resource_dir: &PathBuf,
) -> io::Result<()> {
    static FILES_TO_EXTRACT: LazyLock<HashMap<&str, PluginResource>> = LazyLock::new(|| {
        HashMap::from([
            ("plugin-icon.png", PluginResource::Icon),
            ("plugin-data.json", PluginResource::Data),
        ])
    });
    let reader = Cursor::new(&jar);
    let mut archive = zip::ZipArchive::new(reader)?;

    let mut extracted_count = 0;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        if !file.is_file() {
            continue;
        }

        let res_name = if let Some(resource) =
            FILES_TO_EXTRACT.get(file.name().rsplit('/').next().unwrap_or(""))
        {
            resource.to_string()
        } else {
            continue;
        };

        let dest_path = temp_dir.join(res_name);
        let mut writer = BufWriter::new(File::create(&dest_path)?);

        let mut buffer = [0u8; 4096];
        loop {
            let read = file.read(&mut buffer)?;
            if read == 0 {
                break;
            }

            writer.write_all(&buffer[..read])?;
        }

        extracted_count += 1;
    }

    if extracted_count != FILES_TO_EXTRACT.len() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "the jar has missing data",
        ));
    }

    let dest_path = temp_dir.join(PluginResource::Jar.to_string());
    let mut writer = BufWriter::new(File::create(&dest_path)?);
    writer.write_all(jar.as_ref())?;

    fs::create_dir_all(resource_dir)?;

    for entry in fs::read_dir(temp_dir)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let src = entry.path();
        let dest = resource_dir.join(file_name);
        fs::copy(&src, &dest)
            .map_err(|e| io::Error::new(e.kind(), format!("copy failed: {}", e)))?;

        let _ = fs::remove_file(&src);
    }

    Ok(())
}
