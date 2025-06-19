use crate::data::{PluginResource, PluginResourceType};
use crate::entities::Plugin;
use actix_web::{get, web, Responder};
use data::PluginData;
use serde::Deserialize;
use sqlx::{query, SqlitePool};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::ops::Deref;
use std::path::PathBuf;
use tokio::sync::Mutex;

#[derive(Deserialize)]
struct Filter {
    q: Option<String>,
    tags: Option<String>,
    cat: Option<String>,
}

#[get("")]
pub async fn search(
    query: web::Query<Filter>,
    db_pool: web::Data<Mutex<SqlitePool>>,
) -> actix_web::Result<impl Responder> {
    let tags: HashSet<String> = query
        .tags
        .as_deref()
        .unwrap_or("")
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_string())
        .collect();

    let categories: HashSet<String> = query
        .cat
        .as_deref()
        .unwrap_or("")
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_string())
        .collect();
    
    let q = query.q.as_deref().unwrap_or("");

    let mut sql = String::from(
        r#"
        select distinct p.id, p.name, p.last_version
        from plugin p
        left join plugin_tag t on p.id = t.plugin_id
        left join plugin_category c on p.id = c.plugin_id
        where p.name like ? or p.id like ?
        "#,
    );

    let mut params: Vec<String> = vec![
        format!("%{}%", q),
        format!("%{}%", q)
    ];

    if !tags.is_empty() {
        sql += " or (";
        sql += &tags
            .iter()
            .map(|_| "t.tag_name like ?")
            .collect::<Vec<_>>()
            .join(" or ");
        sql += ")";
        params.extend(tags.iter().map(|tag| format!("%{}%", tag)));
    }

    if !categories.is_empty() {
        sql += " or (";
        sql += &categories
            .iter()
            .map(|_| "c.category_name like ?")
            .collect::<Vec<_>>()
            .join(" or ");
        sql += ")";
        params.extend(categories.iter().map(|cat| format!("%{}%", cat)));
    }

    let guard = db_pool.lock().await;
    let mut query_builder = sqlx::query_as(&sql);
    for param in &params {
        query_builder = query_builder.bind(param);
    }

    let plugins: Vec<Plugin> = query_builder
        .fetch_all(guard.deref())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let data_files = plugins.into_iter().filter_map(|p| {
        let version: data::Version = p.last_version().try_into().ok()?;
        let data = PluginResource::new(p.id(), &version, &PluginResourceType::Data);
        let plugin_data_path = PathBuf::from(data.get_path());

        if !plugin_data_path.exists() {
            return None;
        }

        let reader = BufReader::new(File::open(plugin_data_path).ok()?);
        let plugin_data: PluginData = serde_json::from_reader(reader).ok()?;

        Some(plugin_data)
    })
           .collect::<Vec<_>>();

    let mut response = HashMap::new();
    response.insert("plugins-data", data_files);

    Ok(web::Json(response))
}
