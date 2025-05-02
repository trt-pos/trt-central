use crate::PLUGINS_REPO_DIR;
use crate::data::Metadata;
use data::{PluginData, Version};
use getset::Getters;
use std::{
    fmt::{Display, Formatter},
    io::{Cursor, Read},
};

pub enum PluginResource {
    Jar,
    Icon,
    Data,
}

impl Display for PluginResource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginResource::Jar => write!(f, "jar"),
            PluginResource::Icon => write!(f, "icon"),
            PluginResource::Data => write!(f, "data"),
        }
    }
}

impl TryFrom<&str> for PluginResource {
    type Error = actix_web::error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "jar" => Ok(PluginResource::Jar),
            "icon" => Ok(PluginResource::Icon),
            "data" => Ok(PluginResource::Data),
            _ => Err(actix_web::error::ErrorBadRequest(format!(
                "Invalid resource: {value}. expected jar, icon or data"
            ))),
        }
    }
}

#[derive(Getters)]
pub struct Plugin {
    #[get = "pub"]
    plugin_id: String,
    #[get = "pub"]
    version: Version,
}

impl Plugin {
    pub fn new(plugin_id: &str, version: &str) -> actix_web::Result<Self> {
        let version = if version == "last" {
            let metadata =
                Metadata::load(&plugin_id).map_err(|e| actix_web::error::ErrorNotFound(e))?;
            metadata.versioning().last().to_string()
        } else {
            version.to_string()
        };

        Ok(Self {
            plugin_id: plugin_id.to_string(),
            version: version
                .try_into()
                .map_err(|e: data::Error| actix_web::error::ErrorBadRequest(e.to_string()))?,
        })
    }

    #[inline(always)]
    pub fn new_last_version(plugin_id: &str) -> actix_web::Result<Self> {
        Self::new(plugin_id, "last")
    }

    #[inline(always)]
    pub fn get_plugin_repo_path(&self) -> String {
        format!("{}/{}", *PLUGINS_REPO_DIR, self.plugin_id)
    }

    #[inline(always)]
    pub fn get_resource_path(&self) -> String {
        format!("{}/{}", self.get_plugin_repo_path(), self.version)
    }

    #[inline(always)]
    pub fn get_plugin_resource(&self, resource: &PluginResource) -> String {
        format!("{}/{}", self.get_resource_path(), resource)
    }
}

impl TryFrom<&[u8]> for Plugin {
    type Error = actix_web::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let reader = Cursor::new(&value);
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

            return Plugin::new(plugin_data.id(), &plugin_data.version().to_string());
        }

        Err(actix_web::error::ErrorBadRequest(
            "Invalid plugin data".to_string(),
        ))
    }
}

impl Display for Plugin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.plugin_id, self.version)
    }
}
