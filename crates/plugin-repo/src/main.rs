#![allow(unused)]

use actix_web::web;
use clap::Parser;
use sqlx::migrate::Migrator;
use sqlx::sqlite;
use std::path::PathBuf;
use std::sync::LazyLock;
use tokio::sync::Mutex;

mod controllers;
mod data;
mod middleware;
mod entities;
mod error;

use error::Error;

#[derive(Parser, Debug)]
#[command(name = "plugin-server", about = "TRT Plugins Repository Server")]
struct Args {
    #[arg(
        short = 'p',
        long = "port",
        env = "TRT_PLUGINS_REPO_PORT",
        default_value_t = 1238
    )]
    port: u16,

    #[arg(
        short = 'm',
        long = "max-payload",
        env = "TRT_PLUGINS_REPO_MAX_PAYLOAD",
        default_value_t = 50 * 1024 * 1024
    )]
    max_payload: usize,
    
    #[arg(short = 'a', long = "addrs", env = "TRT_PLUGINS_REPO_ADDRS")]
    addrs: String,

    #[arg(long = "password", env = "TRT_PLUGINS_REPO_PASSWORD")]
    password: String,

    #[arg(short = 'd', long = "dir", env = "TRT_PLUGINS_REPO_DIR")]
    plugins_repo_dir: String,
}

static ARGS: LazyLock<Args> = LazyLock::new(Args::parse);

static PLUGINS_REPO_DIR: LazyLock<&'static str> = LazyLock::new(|| ARGS.plugins_repo_dir.as_str());

static PASSWORD: LazyLock<String> = LazyLock::new(|| "Bearer ".to_owned() + ARGS.password.as_str());

#[tokio::main]
async fn main() -> std::io::Result<()> {
    start_server(ARGS.port, &ARGS.addrs).await
}

async fn start_server(port: u16, addrs: &str) -> std::io::Result<()> {
    let connect_options = sqlite::SqliteConnectOptions::new()
        .filename(PathBuf::from(&ARGS.plugins_repo_dir).join("plugins.db"))
        .create_if_missing(true);

    let db_pool = sqlite::SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(connect_options)
        .await
        .expect("Failed to connect to MariaDB");

    static MIGRATOR: Migrator = sqlx::migrate!();
    MIGRATOR
        .run(&db_pool)
        .await
        .unwrap_or_else(|e| panic!("Failed to run migrations: {}", e));

    println!("Migrations completed");
    
    println!("Starting server on {}:{}", addrs, port);
    
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(web::PayloadConfig::new(ARGS.max_payload))
            .app_data(web::Data::new(Mutex::new(db_pool.clone())))
            .service(
                web::scope("/plugin")
                    .service(controllers::plugin::get_plugin_resource)
                    .service(controllers::plugin::post_plugin)
                    .service(controllers::plugin::put_plugin),
            )
            .service(web::scope("/search").service(controllers::search::search))
    })
    .bind((addrs, port))?
    .run()
    .await
}