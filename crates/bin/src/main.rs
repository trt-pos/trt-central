use actix_web::web;
use sqlx::migrate::Migrator;
use sqlx::mysql;
use std::sync::LazyLock;

mod controllers;
mod entities;

pub static APP_DIR: LazyLock<String> = LazyLock::new(|| {
    let exe_path = std::env::current_exe().expect("The exe path couldn't be found");

    let exe_dir = exe_path.parent().expect("The exe path couldn't be found");

    let app_dir = exe_dir.parent().expect("The app dir couldn't be found");

    let app_dir_string = app_dir
        .to_str()
        .expect("The exe path couldn't be converted to a string");
    app_dir_string.to_string()
});

#[tokio::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    dotenv::dotenv().expect("Failed to load .env file");

    start_server(8000, "0.0.0.0").await
}

async fn start_server(port: u16, addrs: &str) -> std::io::Result<()> {
    let db_pool = mysql::MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to connect to MariaDB");

    static MIGRATOR: Migrator = sqlx::migrate!();
    MIGRATOR
        .run(&db_pool)
        .await
        .unwrap_or_else(|e| panic!("Failed to run migrations: {}", e));

    println!("Migrations completed");
    
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(
                web::scope("/api/v3/theroundtable")
                    .service(web::scope("/update").service(controllers::update::available_update))
                    .service(web::scope("/resource").service(controllers::resource::get_resource))
                    .service(
                        web::scope("/account")
                            .service(controllers::account::has_valid_license)
                            .service(controllers::account::login)
                            .service(controllers::account::validate_license),
                    )
                    .service(web::scope("/remote-supp-request"))
                    .service(web::scope("/app-installation")),
            )
    })
    .bind((addrs, port))?
    .run()
    .await
}
