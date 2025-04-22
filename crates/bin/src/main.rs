#[macro_use]
extern crate rocket;

use rocket::{launch, routes};

mod database;
mod controllers;

#[launch]
async fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("port", 8000))
        .merge(("address", "0.0.0.0"));

    rocket::custom(figment)
        .manage(database::init_pool().await)
        .mount("/api/v1/theroundtable", routes![
            controllers::app_installer
        ])
        .mount("/api/v1/theroundtable/plugins", routes![
            controllers::plugin::plugin_file_response,
            controllers::plugin::need_update,
            controllers::plugin::get_plugins_data
        ])
        .mount("/api/v1/theroundtable/update", routes![
            controllers::update::last_app_version,
            controllers::update::available_update,
            controllers::update::app_zip,
            controllers::app_installer
        ])
        .mount("/api/v1/theroundtable/downloads", routes![
            controllers::download::get_resource
        ])
        .mount("/api/v1/theroundtable/accounts", routes![
            controllers::account::has_valid_license,
            controllers::account::login
        ])
        .mount("/api/v1/theroundtable/licenses", routes![
            controllers::license::validate_license
        ])
}