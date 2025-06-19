use crate::PLUGINS_REPO_DIR;
use data::Version;
use getset::{Getters, Setters};
use serde::Deserialize;
use std::fmt::{Display, Formatter};

pub enum PluginResourceType {
    Jar,
    Icon,
    Data,
}

impl Display for PluginResourceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginResourceType::Jar => write!(f, "jar"),
            PluginResourceType::Icon => write!(f, "icon"),
            PluginResourceType::Data => write!(f, "data"),
        }
    }
}

impl TryFrom<&str> for PluginResourceType {
    type Error = actix_web::error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "jar" => Ok(PluginResourceType::Jar),
            "icon" => Ok(PluginResourceType::Icon),
            "data" => Ok(PluginResourceType::Data),
            _ => Err(actix_web::error::ErrorBadRequest(format!(
                "Invalid resource: {value}. expected jar, icon or data"
            ))),
        }
    }
}

#[derive(Getters, Setters, Deserialize)]
pub struct PluginResource {
    #[getset(get = "pub")]
    plugin_id: String,
    #[getset(get = "pub", set = "pub")]
    version: String,
    #[getset(get = "pub")]
    resource: String,
}

impl PluginResource {
    pub fn new(plugin_id: &str, version: &Version, resource: &PluginResourceType) -> Self {
        Self {
            plugin_id: plugin_id.to_string(),
            version: version.to_string(),
            resource: resource.to_string(),
        }
    }
    #[inline(always)]
    pub fn get_path(&self) -> String {
        format!(
            "{}/{}",
            get_plugin_folder_path(&self.plugin_id, &self.version.to_string()),
            self.resource
        )
    }
}

#[inline(always)]
pub fn get_plugin_folder_path(id: &str, version: &str) -> String {
    format!("{}/{}/{}", *PLUGINS_REPO_DIR, id, version)
}