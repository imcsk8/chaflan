extern crate rocket;
use rocket::fs::FileServer;
use rocket::{launch, routes};
use rocket_dyn_templates::Template;
//https://rocket.rs/guide/v0.5/requests/

pub mod auth;
pub mod claims;
pub mod db;
pub mod models;
pub mod schema;
pub mod events;
pub static STATIC_FILES_DIR: &str = "www/static";

use rocket::catch;
use rocket::serde::json::{Value, json};

#[catch(400)]
fn bad_request() -> Value {
    json!({
        "status": "error",
        "message": "Bad Request. The request body might be too large or incorrectly formatted."
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", rocket::catchers![bad_request])
		.mount("/", routes![auth::login, auth::login_page])
        .mount(
            "/events",
            routes![events::add, events::delete, events::get_json,
                events::get_html, events::list, events::modify, events::upload_image, events::add_page, events::update_page],
        )
        .mount("/public", FileServer::from(STATIC_FILES_DIR))
        .attach(Template::fairing())
        .attach(db::EventDB::fairing())

}
