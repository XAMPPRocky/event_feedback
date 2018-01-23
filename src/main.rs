#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate event_feedback;
extern crate rocket;
extern crate dotenv;
extern crate openssl_probe;

fn main() {
    openssl_probe::init_ssl_cert_env_vars();
    dotenv::dotenv().unwrap();
    event_feedback::rocket().launch();
}
