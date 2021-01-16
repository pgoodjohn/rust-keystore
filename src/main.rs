#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::collections::HashMap;
use std::sync::Mutex;
use rocket::State;
use rocket::http::Status;
use rocket::response::status;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<key>")]
fn retrieve(key: String, db: State<Db>) -> status::Custom<String> {
    let lock: &Db = db.inner();
    match lock.content.lock().unwrap().get(&key)  {
       Some(value) => status::Custom(Status::Ok, value.to_string()),
        _ => status::Custom(Status::NotFound, "".to_string())
    }
}

#[post("/<key>/<value>")]
fn store(key: String, value: String, db: State<Db>) -> status::Created<String> {
    let location = key.clone();
    let content = value.clone();
    let lock: &Db = db.inner();
    lock.content.lock().unwrap().insert(key, value);

    status::Created(location.to_string(), Some(content.to_string()))
}

struct Db {
    content: Mutex<HashMap<String, String>>
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, retrieve, store])
        .manage(Db { content: Mutex::new(HashMap::new())})
}

fn main() {
    rocket().launch();
}


#[cfg(test)]
    mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn it_boots() {
        let client = Client::new(rocket()).expect("Valid rocket instance");
        let mut response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }

    #[test]
    fn it_stores_and_retrieves_a_key() {
        let client = Client::new(rocket()).expect("Valid rocket instance");

        // First there shouldn't be anything stored for key
        let mut empty_response = client.get("/key").dispatch();

        assert_eq!(empty_response.status(), Status::NotFound);
        assert_eq!(empty_response.body_string(), Some("".into()));

        let post_response = client.post("/key/value").dispatch();
       
        assert_eq!(post_response.status(), Status::Created);

        let mut now_it_should_have_the_key = client.get("/key").dispatch();

        assert_eq!(now_it_should_have_the_key.status(), Status::Ok);
        assert_eq!(now_it_should_have_the_key.body_string(), Some("value".into()));
    }

}
