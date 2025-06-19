use crate::Version;
use getset::Getters;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Deserialize, Serialize, Getters)]
pub struct PluginData {
    #[serde(rename = "name")]
    #[getset(get = "pub")]
    name: String,
    #[serde(rename = "id")]
    #[getset(get = "pub")]
    id: String,
    #[serde(rename = "description")]
    #[getset(get = "pub")]
    description: String,
    #[serde(rename = "version")]
    #[getset(get = "pub")]
    version: Version,
    #[serde(rename = "vendor")]
    #[getset(get = "pub")]
    vendor: String,
    #[serde(rename = "vendor-url")]
    #[getset(get = "pub")]
    vendor_url: String,
    #[serde(rename = "tags")]
    #[getset(get = "pub")]
    tags: Option<Vec<String>>,
    #[serde(rename = "categories")]
    #[getset(get = "pub")]
    categories: Option<Vec<String>>,
    #[serde(rename = "dependencies")]
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
    #[serde(rename = "id")]
    pub plugin_id: String,
    #[serde(rename = "version")]
    pub plugin_version: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_plugin_data() {
        let plugin_data = r#"
{
  "name": "Table Drawing",
  "id": "floor-plan",
  "description": "Draw and manage the tables is your establishment",
  "version": "3.0.0",
  "vendor": "Leba Studios - Software Solutions",
  "vendor-url": "https://lebastudios.org",
  "dependencies": [
    {
      "id": "core",
      "version": "3.0.0"
    },
    {
      "id": "cr",
      "version": "3.0.0"
    }
  ]
}
        "#;

        serde_json::from_str::<PluginData>(plugin_data).expect("Should be builded succesfully");
    }
}
