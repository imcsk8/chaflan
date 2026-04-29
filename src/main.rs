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

#[launch]
fn rocket() -> _ {
    rocket::build()
		.mount("/", routes![auth::login])
        .mount(
            "/events",
            routes![events::add, events::delete, events::get_json,
                events::get_html, events::list, events::modify, events::upload_image],
        )
        .mount("/public", FileServer::from(STATIC_FILES_DIR))
        .attach(Template::fairing())
        .attach(db::EventDB::fairing())

}
