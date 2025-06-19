use crate::data::plugin::get_plugin_folder_path;
use crate::data::{PluginResource, PluginResourceType};
use crate::entities::{Category, Entity, Plugin, PluginVersion, Tag};
use actix_web::{get, post, put, web, Responder};
use data::{PluginData, Version};
use sqlx::{Sqlite, Transaction};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufWriter, Cursor, Read, Write};
use std::ops::DerefMut;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::{fs, io};
use tokio::sync::Mutex;

#[get("/{plugin_id}/{version}/{resource}")]
pub async fn get_plugin_resource(
    mut path: web::Path<PluginResource>,
    db_pool: web::Data<Mutex<sqlx::SqlitePool>>,
) -> actix_web::Result<impl Responder> {
    if path.version() == "last" {
        let guard = db_pool.lock().await;
        let plugin = Plugin::get(path.plugin_id(), &guard)
            .await
            .map_err(actix_web::error::ErrorNotFound)?;

        path.set_version(plugin.last_version().to_string());
    }

    let res_path = PathBuf::from(path.get_path());

    if !res_path.exists() {
        return Err(actix_web::error::ErrorNotFound(
            "resource not found in the repository".to_string(),
        ));
    }

    Ok(actix_files::NamedFile::open(res_path))
}

#[post("/")]
pub async fn post_plugin(
    body: web::Bytes,
    db_pool: web::Data<Mutex<sqlx::SqlitePool>>,
    _auth: crate::middleware::PluginPublishingAuthToken,
) -> actix_web::Result<impl Responder> {
    let mut guard = db_pool.lock().await;

    persist_plugin(body, guard.deref_mut(), |plugin| {
        let path_buff = PathBuf::from(get_plugin_folder_path(
            plugin.data.id(),
            &plugin.data.version().to_string(),
        ));

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
    db_pool: web::Data<Mutex<sqlx::SqlitePool>>,
    _auth: crate::middleware::PluginPublishingAuthToken,
) -> actix_web::Result<impl Responder> {
    let mut guard = db_pool.lock().await;

    persist_plugin(body, guard.deref_mut(), |plugin| {
        let plugin_version_dir = PathBuf::from(get_plugin_folder_path(
            plugin.data.id(),
            &plugin.data.version().to_string(),
        ));

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

async fn persist_plugin<F>(
    body: web::Bytes,
    db_pool: &mut sqlx::SqlitePool,
    validate: F,
) -> actix_web::Result<impl Responder + use<F>>
where
    F: Fn(&PluginJar) -> Result<(), actix_web::error::Error>,
{
    let plugin = PluginJar::new(body.as_ref())?;
    validate(&plugin)?;

    let mut transaction = db_pool.begin().await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Failed to begin transaction: {}", e))
    })?;

    if let Err(e) = plugin.persist(&mut transaction).await {
        let _ = transaction.rollback().await;
        return Err(actix_web::error::ErrorInternalServerError(e));
    };

    let temp_dir = std::env::temp_dir().join(format!("trt_plugin_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir)?;

    let plugin_dir = PathBuf::from(get_plugin_folder_path(
        plugin.data.id(),
        &plugin.data.version().to_string(),
    ));

    if let Err(e) = extract_and_save_plugin_from_zip(&temp_dir, body, &plugin_dir).await {
        let _ = fs::remove_dir_all(&temp_dir);
        let _ = transaction.rollback().await;
        return Err(e.into());
    }

    let _ = transaction.commit().await;
    let _ = fs::remove_dir(temp_dir);

    Ok(actix_web::HttpResponse::Ok())
}

async fn extract_and_save_plugin_from_zip(
    temp_dir: &PathBuf,
    jar: web::Bytes,
    resource_dir: &PathBuf,
) -> io::Result<()> {
    static FILES_TO_EXTRACT: LazyLock<HashMap<&str, PluginResourceType>> = LazyLock::new(|| {
        HashMap::from([
            ("plugin-icon.png", PluginResourceType::Icon),
            ("plugin-data.json", PluginResourceType::Data),
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

    let dest_path = temp_dir.join(PluginResourceType::Jar.to_string());
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

struct PluginJar<'b> {
    jar_bytes: &'b [u8],
    data: PluginData,
}

impl<'b> PluginJar<'b> {
    fn new(jar_bytes: &'b [u8]) -> Result<Self, actix_web::Error> {
        let reader = Cursor::new(jar_bytes);
        let mut archive = zip::ZipArchive::new(reader).map_err(|e| {
            actix_web::error::ErrorBadRequest(format!("Failed to open the plugin jar: {}", e))
        })?;

        for i in 0..archive.len() {
            let mut file = if let Ok(file) = archive.by_index(i) {
                file
            } else {
                continue;
            };

            if !file.is_file() || file.name().rsplit('/').next().unwrap_or("") != "plugin-data.json"
            {
                continue;
            }

            let mut plugin_data_json = String::new();
            if file.read_to_string(&mut plugin_data_json).is_err() {
                continue;
            }

            let plugin_data =
                serde_json::from_str::<PluginData>(&plugin_data_json).map_err(|e| {
                    actix_web::error::ErrorBadRequest(format!("unable to parse plugin data: {e}"))
                })?;

            return Ok(Self {
                jar_bytes,
                data: plugin_data,
            });
        }

        Err(actix_web::error::ErrorBadRequest(
            "Invalid plugin data".to_string(),
        ))
    }

    async fn persist(&self, executor: &mut Transaction<'_, Sqlite>) -> Result<(), crate::Error> {
        if let Some(categories) = self.data.categories() {
            for category in categories {
                let category = Category::new(category);
                if !category.exists(&mut **executor).await? {
                    category.insert(&mut **executor).await?;
                }
            }
        }

        if let Some(tags) = self.data.tags() {
            for tag in tags {
                let tag = Tag::new(tag);
                if !tag.exists(&mut **executor).await? {
                    tag.insert(&mut **executor).await?;
                }
            }
        }

        let plugin: Option<Plugin> =
            sqlx::query_as("select id, name, last_version from plugin where id = ?")
                .bind(self.data.id())
                .fetch_optional(&mut **executor)
                .await?;

        if let Some(plugin) = plugin {
            let old_version: Version = plugin
                .last_version()
                .try_into()
                .map_err(|_| sqlx::Error::Decode("Invalid version format".into()))?;

            if old_version < *self.data.version() {
                let plugin = Plugin::new(self.data.id(), self.data.name(), self.data.version());
                plugin.update(&mut **executor).await?;

                sqlx::query("delete from plugin_tag where plugin_id = ?")
                    .bind(self.data.id())
                    .execute(&mut **executor)
                    .await?;

                sqlx::query("delete from plugin_category where plugin_id = ?")
                    .bind(self.data.id())
                    .execute(&mut **executor)
                    .await?;

                if let Some(categories) = self.data.categories() {
                    for category in categories {
                        sqlx::query(
                            "insert into plugin_category (plugin_id, category_name) values (?, ?)",
                        )
                        .bind(self.data.id())
                        .bind(category)
                        .execute(&mut **executor)
                        .await?;
                    }
                }

                if let Some(tags) = self.data.tags() {
                    for tag in tags {
                        sqlx::query("insert into plugin_tag (plugin_id, tag_name) values (?, ?)")
                            .bind(self.data.id())
                            .bind(tag)
                            .execute(&mut **executor)
                            .await?;
                    }
                }
            };
        } else {
            let plugin = Plugin::new(self.data.id(), self.data.name(), self.data.version());
            plugin.insert(&mut **executor).await?;
        }

        let plugin_version = PluginVersion::new(self.data.id(), self.data.version());
        if !plugin_version.exists(&mut **executor).await? {
            plugin_version.insert(&mut **executor).await?;
        }

        Ok(())
    }
}

impl Display for PluginJar<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.data.id(), self.data.version())
    }
}
