use serde::{Deserialize, Serialize};

use crate::Version;
use serde::ser::SerializeMap;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;
use std::hash::{Hash, Hasher};
// region: PluginData

#[derive(Deserialize, Serialize)]
pub struct PluginData {
    #[serde(rename = "pluginName")]
    plugin_name: String,
    #[serde(rename = "pluginIcon")]
    plugin_icon: String,
    #[serde(rename = "pluginId")]
    plugin_id: String,
    #[serde(rename = "pluginDescription")]
    plugin_description: String,
    #[serde(rename = "pluginVersion")]
    plugin_version: String,
    #[serde(rename = "pluginVendor")]
    plugin_vendor_url: String,
    #[serde(rename = "pluginRequiredCoreVersion")]
    plugin_required_core_version: String,
    #[serde(rename = "pluginDependencies")]
    plugin_dependency: Vec<PluginDependency>,
}

impl PluginData {
    pub fn version(self: &Self) -> Version {
        Version::new(self.plugin_version.clone())
    }
}

impl PartialEq for PluginData {
    fn eq(&self, other: &Self) -> bool {
        self.plugin_name == other.plugin_name
    }
}

impl Hash for PluginData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.plugin_name.hash(state)
    }
}

// endregion: PluginData

// region: PluginDependency

#[derive(Deserialize, Serialize)]
pub struct PluginDependency {
    #[serde(rename = "pluginId")]
    pub plugin_name: String,
    #[serde(rename = "pluginVersion")]
    pub plugin_version: String,
}

// endregion: PluginDependency

// region: Account

#[derive(FromRow, Deserialize, Serialize)]
pub struct Account {
    id: i32,
    password: String,
    email: String,
}

// endregion: Account

// region: License

#[derive(FromRow)]
pub struct License {
    id: String,
    r#type: String,
    end_date: DateTime<Utc>,
    owner_account_id: i32,
}

impl License {
    pub fn is_valid(self: &Self) -> bool {
        self.end_date > Utc::now()
    }
}

impl Serialize for License {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("type", &self.r#type)?;
        map.serialize_entry("end_date", &self.end_date.to_string())?;
        map.serialize_entry("account_id", &self.owner_account_id)?;
        map.end()
    }
    
}

// endregion: License
