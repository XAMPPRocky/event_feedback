use std::io::Cursor;

use diesel;
use serde_json;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::request::{Form, Request};
use rocket::response::{self, Response, Responder};
use rocket::response::content::Json;
use uuid::Uuid;

use admin::AdminKey;
use postgres::Postgres;
use schema::feedback;
use super::RequestResult;
use event::Event;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Queryable, Serialize)]
pub struct Feedback {
    pub id: Uuid,
    pub secret: String,
    pub body: String,
}

#[derive(Clone, Debug, Insertable)]
#[table_name = "feedback"]
struct NewFeedback<'a> {
    id: &'a Uuid,
    secret: &'a String,
    body: &'a str,
}

#[derive(FromForm)]
pub struct FeedbackForm {
    secret: String,
    body: String,
}

impl<'r> Responder<'r> for Feedback {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .status(Status::Ok)
            .sized_body(Cursor::new(serde_json::to_string(&self).unwrap()))
            .ok()
    }
}

pub struct MultipleFeedback(Vec<Feedback>);

impl<'r> Responder<'r> for MultipleFeedback {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .status(Status::Ok)
            .sized_body(Cursor::new(serde_json::to_string(&self.0).unwrap()))
            .ok()
    }
}

#[post("/", data = "<form>")]
pub fn create(form: Form<FeedbackForm>, postgres: Postgres)
    -> RequestResult<Json<Feedback>>
{
    let form = form.get();

    {
        use schema::events::dsl::*;
        let _: Event = unwrap!(
            events.filter(id.eq(&form.secret)).first::<Event>(&*postgres)
        );
    }

    let feedback = {
        use schema::feedback;

        let id = Uuid::new_v4();

        let new_feedback = NewFeedback {
            id: &id,
            secret: &form.secret,
            body: &form.body,
        };

        unwrap!(
            diesel::insert_into(feedback::table)
            .values(&new_feedback)
            .on_conflict_do_nothing()
            .get_result::<Feedback>(&*postgres)
        )
    };

    Ok(Json(feedback))
}

#[get("/<secret_input>")]
pub fn get(_admin: AdminKey, secret_input: String, postgres: Postgres)
    -> RequestResult<Json<MultipleFeedback>>
{
    use schema::feedback::dsl::*;
    let feedback_items = unwrap!(
        feedback.filter(secret.eq(&secret_input))
                .get_results::<Feedback>(&*postgres)
    );

    Ok(Json(MultipleFeedback(feedback_items)))
}

#[delete("/<id_input>")]
pub fn delete(_admin: AdminKey,
              id_input: String,
              postgres: Postgres)
    -> RequestResult<Json<Feedback>>
{
    use schema::feedback::dsl::*;

    let id_input = unwrap!(Uuid::parse_str(&id_input));

    Ok(Json(unwrap!(diesel::delete(feedback.find(&id_input))
                           .get_result::<Feedback>(&*postgres))))
}
