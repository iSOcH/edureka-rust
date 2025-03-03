use std::env;

use diesel::{Connection, PgConnection};
use diesel::prelude::*;
use dotenvy::dotenv;
use models::{NewUser, User};
use mycounterfairing::MyCounterFairing;
use rocket::http::CookieJar;
use crate::schema::users::dsl::users;

#[macro_use] extern crate rocket;

static COOKIE_NAME: &str = "mycookie";

mod mycounterfairing;
mod models;
mod schema;

#[launch]
fn rocket() -> _ {
    // note that middlewares seemingly need a Fairing impl, the course does not mention this and the given code does not compile
    // instead I used https://www.shuttle.dev/blog/2022/08/04/middleware-in-rust#rocket as a starting point
    let app = rocket::build()
        .attach(MyCounterFairing::new())
        .mount("/", routes![home, set_cookie, user_get, submit, user_update_name]);
    app
}

pub fn connect_db() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
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
fn user_get(id: i32) -> String {
    let connection = &mut connect_db();
    let result = users.filter(schema::users::id.eq(id)).select(User::as_select()).load(connection).expect("failed to query");
    result.first().map(|u| u.name.clone()).unwrap_or_else(|| "not found".to_owned())
}

#[post("/users", data = "<name>")]
fn submit(name: String) -> String {
    let connection = &mut connect_db();
    
    let user = NewUser {
        name: &name,
        email: "not_supported"
    };

    let created_user = diesel::insert_into(schema::users::table).values(&user).returning(models::User::as_returning()).get_result(connection);
    created_user.unwrap().name
}

#[put("/users/<id>/name", data = "<_name>")]
fn user_update_name(id: i32, _name: String) -> String {
    use self::schema::users::dsl::{users, name};
    let connection = &mut connect_db();
    
    let updated_user = diesel::update(users.find(id)).set(name.eq(_name)).returning(User::as_returning()).get_result(connection);
    updated_user.map(|u| u.name.clone()).expect("failed to update")
}