use crate::claims::Claims;
use crate::db::*;
use crate::models::Event;
use crate::schema::events::dsl::*;
use diesel::prelude::*;
use rocket::response::status::NotFound;
use rocket::response::{status::Created, Debug};
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::{delete, get, post};
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

//https://api.rocket.rs/v0.5/rocket_sync_db_pools/

/// Show the list of events in HTML
#[get("/")]
pub async fn list(tdb: EventDB) -> Template {
    let results = tdb
        .run(move |connection| {
            crate::schema::events::dsl::events
                .load::<Event>(connection)
                .expect("Error loading events")
        })
        .await;
    Template::render("events", context! {events: &results, count: results.len()})
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
