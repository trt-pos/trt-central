use crate::Version;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Deserialize, Serialize)]
pub struct PluginData {
    #[serde(rename = "pluginName")]
    plugin_name: String,
    #[serde(rename = "pluginId")]
    plugin_id: String,
    #[serde(rename = "pluginDescription")]
    plugin_description: String,
    #[serde(rename = "pluginVersion")]
    plugin_version: Version,
    #[serde(rename = "pluginVendor")]
    plugin_vendor: String,
    #[serde(rename = "pluginVendorUrl")]
    plugin_vendor_url: String,
    #[serde(rename = "pluginDependencies")]
    plugin_dependencies: Vec<PluginDependency>,
}

impl PluginData {
    pub fn version(&self) -> &Version {
        &self.plugin_version
    }

    pub fn id(&self) -> &str {
        &self.plugin_id
    }
}

impl PartialEq for PluginData {
    fn eq(&self, other: &Self) -> bool {
        self.plugin_name == other.plugin_name
    }
}

impl Hash for PluginData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.plugin_id.hash(state)
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
  "pluginId": "plugin-table-drawing",
  "pluginDescription": "Draw and manage the tables is your establishment",
  "pluginVersion": "3.0.0",
  "pluginVendor": "Leba Studios - Software Solutions",
  "pluginVendorUrl": "https://lebastudios.org",
  "pluginDependencies": [
    {
      "pluginId": "desktop-app",
      "pluginVersion": "3.0.0"
    },
    {
      "pluginId": "plugin-cash-register",
      "pluginVersion": "3.0.0"
    }
  ]
}
        "#;

        serde_json::from_str::<PluginData>(plugin_data).expect("Should be builded succesfully");
    }
}
