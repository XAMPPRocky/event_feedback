use std::env;

use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

lazy_static! {
    static ref ADMIN_KEY: String = env::var("ADMIN_KEY").unwrap();
}

#[derive(Clone, Debug)]
pub struct AdminKey;

impl<'a, 'r> FromRequest<'a, 'r> for AdminKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AdminKey, ()> {

        let key = match request.headers().get_one("Authorization") {
            Some(key) => key,
            None => {
                debug!("No Authorization Bearer Key");
                return Outcome::Failure((Status::Unauthorized, ()))
            },
        };

        let value: Vec<_> = key.split_whitespace().collect();

        if value.len() != 2 {
            debug!("Invalid Authorization Bearer Key: {:?}", value);
            return Outcome::Failure((Status::Unauthorized, ()));
        }


        if value[1] == *ADMIN_KEY {
            Outcome::Success(AdminKey)
        } else {
            debug!("Invalid Admin Key");
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}
