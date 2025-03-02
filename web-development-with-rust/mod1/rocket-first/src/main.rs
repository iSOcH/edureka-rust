use mycounterfairing::MyCounterFairing;
use rocket::http::CookieJar;

#[macro_use] extern crate rocket;

static COOKIE_NAME: &str = "mycookie";

mod mycounterfairing;

#[launch]
fn rocket() -> _ {
    // note that middlewares seemingly need a Fairing impl, the course does not mention this and the given code does not compile
    // instead I used https://www.shuttle.dev/blog/2022/08/04/middleware-in-rust#rocket as a starting point
    let app = rocket::build()
        .attach(MyCounterFairing::new())
        .mount("/", routes![home, set_cookie, user_get, submit, user_update_name]);
    app
}

// I fear the video is outdated. it uses a `Cookies` and `Request` type here, but both seem to not be compatible (no impl for FromRequest)
#[get("/")]
fn home(cookies: &CookieJar) -> String {
    let cookie = cookies.get(COOKIE_NAME).map(|c| c.value());
    format!("Hello, your cookie contains {cookie:?}")
}

#[get("/set-cookie")]
fn set_cookie(cookies: &CookieJar) -> String {
    cookies.add((COOKIE_NAME, "foo"));
    format!("Cookie set")
}

#[get("/users/<id>")]
fn user_get(id: u32) -> String {
    format!("Getting user with id {id}...")
}

#[post("/submit", data = "<input>")]
fn submit(input: String) -> String {
    format!("Received {input}\n")
}

#[put("/users/<id>/name", data = "<name>")]
fn user_update_name(id: u32, name: String) -> String {
    let user = User {
        id: id,
        name: name.into_boxed_str()
    };
    user.name.into_string()
}

struct User {
    id: u32,
    name: Box<str>
}