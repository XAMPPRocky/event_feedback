//#![cfg_attr(test, deny(warnings))]
#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
extern crate publicsuffix;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rand;
extern crate ring;
extern crate rocket;
extern crate serde;
extern crate serde_json;
extern crate uuid;

#[macro_use] mod macros;
mod admin;
mod postgres;
mod schema;
pub mod event;
pub mod feedback;

use rocket::response::status::BadRequest;

pub type RequestResult<T> = ::std::result::Result<T, BadRequest<String>>;

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(postgres::init_pool())
        .mount("/events", routes![event::create, event::delete])
        .mount("/feedback", routes![feedback::create,
                                    feedback::get,
                                    feedback::delete])
}

