use std::fmt::format;

use rocket::serde::{json::Json, Serialize};

#[macro_use]
extern crate rocket;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Response<T> {
    message: String,
    ok: bool,
    data: Option<T>
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct UserAccount {
    username: String,
    email: String,
}

#[get("/health")]
fn health() -> Json<Response<String>> {
    let resp = Response {
        message: String::from("server is up"),
        ok: bool::from(true),
        data: None,
    };

    return Json(resp);
}

#[get("/")]
fn get_users() -> Json<Response<Vec<UserAccount>>> {
    let mut users: Vec<UserAccount> = Vec::new();
    let mut i = 0;
    while i <= 25 {
        let username = format!("user-{}", i);
        let email = format!("email-{}", i);

        users.push(UserAccount { username: username, email: email });
        i += 1;
    }

    let resp = Response {
        message: String::from("this is a list users API"),
        ok: bool::from(true),
        data: Some(users),
    };

    return Json(resp);
}

#[get("/detail/<username>")]
fn user_detail(username: &str) -> Json<Response<UserAccount>> {
    let resp = Response {
        message: "".to_string(),
        ok: bool::from(true),
        data: Some(UserAccount{
            email: username.to_string(),
            username: username.to_string(),
        }),
    };

    return Json(resp);
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![health])
        .mount("/users", routes![get_users, user_detail])
}
