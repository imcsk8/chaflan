use crate::claims::Claims;
use crate::db::*;
use crate::models::Event;
use crate::schema::events::dsl::*;
use diesel::prelude::*;
use rocket::response::status::NotFound;
use rocket::response::{status::Created, Debug};
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::{delete, get, post, put};
use rocket::fs::TempFile;
use rocket_dyn_templates::{context, Template};
use rocket_sync_db_pools::diesel;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

/// Creates an event
#[post("/add", format = "json", data = "<arg_event>")]
pub async fn add(arg_event: Json<Event>, _user: Claims, tdb: EventDB) -> Result<Created<Json<Uuid>>> {
    let mut new_event: Event = arg_event.into_inner();
    new_event.id = Uuid::new_v4();
    let ret_id = new_event.id.clone();
    let _event_id = tdb
        .run(move |conn| {
            diesel::insert_into(crate::schema::events::dsl::events)
                .values(&new_event)
                .execute(conn)
                .expect("Error saving new event");
        })
        .await;

    Ok(Created::new("/").body(Json(ret_id)))
}

/// Modifies an event
#[put("/<eventid>", format = "json", data = "<arg_event>")]
pub async fn modify(
    eventid: Uuid,
    arg_event: Json<Event>,
    _user: Claims,
    tdb: EventDB,
) -> Result<Json<Event>, NotFound<String>> {
    let mut updated_event = arg_event.into_inner();
    updated_event.id = eventid;

    let result = tdb
        .run(move |conn| {
            diesel::update(crate::schema::events::dsl::events.filter(crate::schema::events::dsl::id.eq(eventid)))
                .set(&updated_event)
                .get_result::<Event>(conn)
        })
        .await;

    match result {
        Ok(event) => Ok(Json(event)),
        Err(_) => Err(NotFound(format!("Could not find event: {}", eventid))),
    }
}

/// Uploads an image for an event
#[post("/<eventid>/image", data = "<file>")]
pub async fn upload_image(
    eventid: Uuid,
    mut file: TempFile<'_>,
    _user: Claims,
    tdb: EventDB,
) -> std::result::Result<Json<String>, Debug<std::io::Error>> {
    let filename = format!("{}_{}", eventid, file.name().unwrap_or("image.png"));
    let path = std::path::Path::new(crate::STATIC_FILES_DIR).join("uploads").join(&filename);

    std::fs::create_dir_all(path.parent().unwrap()).unwrap();

    file.copy_to(&path).await.map_err(Debug)?;

    let db_path = format!("/public/uploads/{}", filename);
    let db_path_clone = db_path.clone();

    tdb.run(move |conn| {
        diesel::update(crate::schema::events::dsl::events.filter(crate::schema::events::dsl::id.eq(eventid)))
            .set(crate::schema::events::dsl::image.eq(db_path_clone))
            .execute(conn)
            .expect("Error updating event image");
    }).await;

    Ok(Json(format!("Image uploaded: {}", db_path)))
}


//https://api.rocket.rs/v0.5/rocket_sync_db_pools/

/// Show the list of events in HTML
#[get("/")]
pub async fn list(user: Option<Claims>, tdb: EventDB) -> Template {
    let results = tdb
        .run(move |connection| {
            crate::schema::events::dsl::events
                .load::<Event>(connection)
                .expect("Error loading events")
        })
        .await;
    Template::render("events", context! {
        events: &results, 
        count: results.len(),
        is_logged_in: user.is_some()
    })
}

/// Get a event and returns it as a JSON object
#[get("/<eventid>", format="json", rank = 1)]
pub async fn get_json(eventid: Uuid, tdb: EventDB) -> Result<Json<Vec<Event>>, NotFound<String>> {
    let results = tdb
        .run(move |connection| {
            crate::schema::events::dsl::events
                .filter(id.eq(eventid))
                .load::<Event>(connection)
                .expect("Error loading events")
        })
        .await;
    if results.len() > 0 {
        Ok(Json(results))
    } else {
        Err(NotFound(format!("Could not find event: {}", eventid)))
    }
}

/// Get a event and returns it as a JSON object
#[get("/<eventid>", format="text/html", rank=2)]
pub async fn get_html(eventid: Uuid, tdb: EventDB) -> Result<Json<Vec<Event>>, NotFound<String>> {
    let results = tdb
        .run(move |connection| {
            crate::schema::events::dsl::events
                .filter(id.eq(eventid))
                .load::<Event>(connection)
                .expect("Error loading events")
        })
        .await;
    if results.len() > 0 {
        Ok(Json(results))
    } else {
        Err(NotFound(format!("Could not find event: {}", eventid)))
    }
}

/// Remove a event
#[delete("/<eventid>")]
pub async fn delete(
    eventid: Uuid,
    _user: Claims,
    tdb: EventDB,
) -> Result<Json<String>, NotFound<String>> {
    let results = tdb
        .run(move |connection| {
            diesel::delete(crate::schema::events::dsl::events.filter(id.eq(eventid)))
                .execute(connection)
        })
        .await;
    if results.unwrap() == 1 {
        Ok(Json(format!("{} deleted", eventid)))
    } else {
        Err(NotFound(format!("Could not find event: {}", eventid)))
    }
}

/// Serve the page to add a new event
#[get("/add_page")]
pub async fn add_page(_user: Claims) -> Template {
    Template::render("add_event", context! {})
}
