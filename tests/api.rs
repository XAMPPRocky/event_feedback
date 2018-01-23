extern crate event_feedback;
extern crate rocket;
extern crate dotenv;
extern crate serde_json;
#[macro_use] extern crate lazy_static;

use std::env;

use rocket::http::{ContentType, Header, Status};
use rocket::local::Client;
use event_feedback::event::Event;
use event_feedback::feedback::Feedback;

lazy_static! {
    static ref AUTH_HEADER: Header<'static> = {
        Header::new("Authorization", format!("Bearer {}", env::var("ADMIN_KEY").unwrap()))
    };
}

#[test]
fn events() {
    dotenv::dotenv().unwrap();
    let client = Client::new(event_feedback::rocket()).expect("valid rocket instance");
    let event = create_event(&client);
    let feedback = create_feedback(&client, &event.id);
    let feedback_items = get_feedback(&client, &feedback.secret);
    assert_eq!(feedback, feedback_items[0]);

    delete_feedback(&client, feedback);
    delete_event(&client, event);
}

fn create_event(client: &Client) -> Event {
    let mut response = client.post("/events")
        .header(ContentType::Form)
        .header(AUTH_HEADER.clone())
        .body("name=RustConf 2018")
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    let body = response.body_string();
    assert!(body.is_some());

    serde_json::from_str(&body.unwrap()).unwrap()
}

fn delete_event(client: &Client, event: Event) {
    let mut response = client.delete(format!("/events/{}", event.id))
        .header(AUTH_HEADER.clone())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let deleted = serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(event, deleted);
}

fn create_feedback(client: &Client, secret: &str) -> Feedback {
    let mut response = client.post("/feedback")
        .header(ContentType::Form)
        .header(AUTH_HEADER.clone())
        .body(format!("secret={}&body=Nice pizza", secret))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    let body = response.body_string();
    assert!(body.is_some());

    serde_json::from_str(&body.unwrap()).unwrap()
}

fn get_feedback(client: &Client, secret: &str) -> Vec<Feedback> {
    let mut response = client.get(format!("/feedback/{}", secret))
        .header(ContentType::Form)
        .header(AUTH_HEADER.clone())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    let body = response.body_string();
    assert!(body.is_some());

    serde_json::from_str(&body.unwrap()).unwrap()
}

fn delete_feedback(client: &Client, feedback: Feedback) {
    let mut response = client.delete(format!("/feedback/{}", feedback.id))
        .header(AUTH_HEADER.clone())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let deleted = serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(feedback, deleted);
}

