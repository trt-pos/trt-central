use crate::data::{Plugin, PluginResource};
use actix_web::{get, web, Responder};
use data::PluginData;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Deserialize)]
struct QueryParams {
    q: Option<String>,
    tags: Option<String>,
    cat: Option<String>,
}

#[get("/all")]
pub async fn get_all(query: web::Query<QueryParams>) -> actix_web::Result<impl Responder> {
    let _tags: HashSet<String> = if let Some(tags) = &query.tags {
        tags.split(',').map(|s| s.to_string()).collect()
    } else {
        HashSet::new()
    };

    let plugins_dir = PathBuf::from(*crate::PLUGINS_REPO_DIR);

    let plugins_data_files = fs::read_dir(&plugins_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;

            if entry.path().is_file() {
                return None;
            }

            let plugin_name = entry.file_name();
            let plugin_name = plugin_name.to_str()?;

            let plugin = Plugin::new_last_version(plugin_name).ok()?;
            let plugin_data_path = PathBuf::from(plugin.get_plugin_resource(&PluginResource::Data));

            if !plugin_data_path.exists() {
                return None;
            }

            let reader = BufReader::new(File::open(plugin_data_path).ok()?);
            let plugin_data: PluginData = serde_json::from_reader(reader).ok()?;

            Some(plugin_data)
        })
        .collect::<Vec<_>>();

    let mut response = HashMap::new();
    response.insert("plugins-data", plugins_data_files);

    Ok(web::Json(response))
}
