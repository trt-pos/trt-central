use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use server_side::entities::PluginData;
use server_side::{Version, APP_DIR};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[get("/<plugin_id>")]
pub async fn plugin_file_response(plugin_id: &str) -> Option<NamedFile> {
    let path_file = APP_DIR.to_string() + "/plugins";
    let path_buff = PathBuf::from(path_file).join(plugin_id);

    NamedFile::open(path_buff).await.ok()
}

#[get("/<plugin_id>/updateable?<version>")]
pub async fn need_update(plugin_id: &str, version: &str) -> Json<HashMap<String, bool>> {
    let last_version = extract_plugin_version(plugin_id);

    let response = Version::new(version.to_string()).cmp(&last_version) == std::cmp::Ordering::Less;

    Json(HashMap::from([
        ("response".to_string(), response),
        ("succesfull".to_string(), true),
    ]))
}

#[get("/pluginsData")]
pub async fn get_plugins_data() -> Json<Vec<PluginData>> {
    let plugins_dir = PathBuf::from(APP_DIR.to_string() + "/plugins");

    let plugins_data_files = fs::read_dir(&plugins_dir)
        .unwrap()
        // Filtrar los resultados para quedarse solo con archivos que terminan en ".json".
        .filter_map(|entry| {
            let entry = entry.ok().unwrap(); // Ignora cualquier error en los archivos individuales.
            let path = entry.path();

            if path.extension()? == "json" {
                let plugin_data =
                    fs::read_to_string(&path).expect("Should read the plugin data file");

                let plugin_data: PluginData = serde_json::from_str(plugin_data.as_str())
                    .expect(format!("Should be builded succesfully: {:?}", path).as_str());
                Some(plugin_data)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Json(plugins_data_files)
}

// region: AUXILIARY FUNCTIONS

fn extract_plugin_version(plugin_id: &str) -> Version {
    let plugin_data_path =
        PathBuf::from(APP_DIR.to_string() + "/plugins/" + plugin_id + ".jar.json");

    let plugin_data =
        fs::read_to_string(plugin_data_path).expect("Should read the plugin data file");

    let plugin_data: PluginData =
        serde_json::from_str(plugin_data.as_str()).expect("Should be built successfully");

    plugin_data.version()
}

// endregion: AUXILIARY FUNCTIONS

#[cfg(test)]
mod tests {
    use server_side::entities::PluginData;

    #[test]
    fn test_extract_plugin_data() {
        let plugin_data = r#"
{
  "pluginName": "Table Drawing",
  "pluginId": "plugin-table-drawing",
  "pluginIcon": "table",
  "pluginDescription": "Draw and manage the tables is your establishment",
  "pluginVersion": "1.0.0",
  "pluginVendor": "Leba Studios - Software Solutions",
  "pluginVendorUrl": "https://lebastudios.org",
  "pluginRequiredCoreVersion": "2.0.0",
  "pluginDependencies": [
    {
      "pluginId": "plugin-cash-register",
      "pluginVersion": "1.0.0"
    }
  ]
}
        "#;

        serde_json::from_str::<PluginData>(plugin_data).expect("Should be builded succesfully");
    }
}