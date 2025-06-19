use crate::Version;
use getset::Getters;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Deserialize, Serialize, Getters)]
pub struct PluginData {
    #[serde(rename = "pluginName")]
    #[getset(get = "pub")]
    name: String,
    #[serde(rename = "pluginId")]
    #[getset(get = "pub")]
    id: String,
    #[serde(rename = "pluginDescription")]
    #[getset(get = "pub")]
    description: String,
    #[serde(rename = "pluginVersion")]
    #[getset(get = "pub")]
    version: Version,
    #[serde(rename = "pluginVendor")]
    #[getset(get = "pub")]
    vendor: String,
    #[serde(rename = "pluginVendorUrl")]
    #[getset(get = "pub")]
    vendor_url: String,
    #[serde(rename = "tags")]
    #[getset(get = "pub")]
    tags: Option<Vec<String>>,
    #[serde(rename = "categories")]
    #[getset(get = "pub")]
    categories: Option<Vec<String>>,
    #[serde(rename = "pluginDependencies")]
    #[getset(get = "pub")]
    dependencies: Vec<PluginDependency>,
}

impl PartialEq for PluginData {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for PluginData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

#[derive(Deserialize, Serialize)]
pub struct PluginDependency {
    #[serde(rename = "pluginId")]
    pub plugin_id: String,
    #[serde(rename = "pluginVersion")]
    pub plugin_version: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_plugin_data() {
        let plugin_data = r#"
{
  "pluginName": "Table Drawing",
  "pluginId": "floor-plan",
  "pluginDescription": "Draw and manage the tables is your establishment",
  "pluginVersion": "3.0.0",
  "pluginVendor": "Leba Studios - Software Solutions",
  "pluginVendorUrl": "https://lebastudios.org",
  "pluginDependencies": [
    {
      "pluginId": "core",
      "pluginVersion": "3.0.0"
    },
    {
      "pluginId": "cr",
      "pluginVersion": "3.0.0"
    }
  ]
}
        "#;

        serde_json::from_str::<PluginData>(plugin_data).expect("Should be builded succesfully");
    }
}
