use rocket::{get, launch, routes};

#[launch]
fn rocket() -> _ {
    let app = rocket::build().mount("/", routes![home, get_user]);
    app
}

#[get("/")]
fn home() -> &'static str {
    "Hello, world!"
}

#[get("/users/<id>")]
fn get_user(id: u32) -> String {
    format!("Getting user with id {id}...")
}