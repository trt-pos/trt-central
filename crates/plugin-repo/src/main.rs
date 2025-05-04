use actix_web::web;
use clap::Parser;
use std::sync::LazyLock;

mod controllers;
mod data;
mod middleware;

#[derive(Parser, Debug)]
#[command(name = "plugin-server", about = "Servidor de plugins TRT")]
struct Args {
    #[arg(
        short = 'p',
        long = "port",
        env = "TRT_PLUGINS_REPO_PORT",
        default_value_t = 1238
    )]
    port: u16,

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
    println!("Starting server on {}:{}", addrs, port);
    
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .service(
                web::scope("/plugin")
                    .service(controllers::plugin::get_plugin_resource)
                    .service(controllers::plugin::post_plugin)
                    .service(controllers::plugin::put_plugin),
            )
            .service(web::scope("/data").service(controllers::data::get_all))
    })
    .bind((addrs, port))?
    .run()
    .await
}
