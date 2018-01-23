use std::io::Cursor;

use diesel;
use serde_json;
use diesel::prelude::*;
use rand::{self, Rng};
use rocket::http::Status;
use rocket::request::{Form, Request};
use rocket::response::{self, Response, Responder};
use rocket::response::content::Json;

use admin::AdminKey;
use postgres::Postgres;
use schema::events;
use super::RequestResult;

lazy_static! {
    static ref WORDS: Vec<String> = {
        use std::fs::File;
        use std::io::Read;

        let contents = {
            let mut f = File::open("/usr/share/dict/words").unwrap();
            let mut c = String::new();
            f.read_to_string(&mut c).unwrap();
            c
        };

        contents.lines().map(|s| s.to_owned()).collect()
    };

    static ref WORD_LENGTH: usize = WORDS.len();
}

#[derive(Clone, Debug, Eq, PartialEq, Queryable, Identifiable, Serialize, Deserialize)]
#[primary_key(id)]
pub struct Event {
    pub id: String,
    pub name: String,
}

impl<'r> Responder<'r> for Event {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .status(Status::Ok)
            .sized_body(Cursor::new(serde_json::to_string(&self).unwrap()))
            .ok()
    }
}

#[derive(Insertable)]
#[table_name="events"]
struct NewEvent<'a> {
    id: &'a str,
    name: &'a str,
}

#[derive(FromForm)]
pub struct EventForm {
    name: String,
}

#[post("/", data = "<event>")]
pub fn create(_admin: AdminKey, event: Form<EventForm>, postgres: Postgres)
    -> RequestResult<Json<Event>>
{
    let event = event.get();
    let mut rng = rand::thread_rng();
    let mut words = Vec::with_capacity(3);

    for _ in 0..3 {
        words.push(WORDS[rng.gen_range(0, *WORD_LENGTH)].clone());
    }

    let event = NewEvent {
        id: &words.join("_"),
        name: &event.name,
    };

    let event = unwrap!(diesel::insert_into(events::table)
            .values(&event)
            .on_conflict_do_nothing()
            .get_result::<Event>(&*postgres));

    Ok(Json(event))
}

#[delete("/<id>")]
pub fn delete(_admin: AdminKey, id: String, postgres: Postgres)
    -> RequestResult<Json<Event>>
{
    use schema::events::dsl::events;

    Ok(Json(unwrap!(diesel::delete(events.find(&id)).get_result::<Event>(&*postgres))))
}
