mod server;
mod users;
mod utils;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![server::server_router::health])
        .mount(
            "/users",
            routes![
                users::user_router::get_users,
                users::user_router::user_detail
            ],
        )
}
