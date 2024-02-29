mod base_schema;
mod responses;
mod users;
mod server;
mod libraries;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    let pool = libraries::db_connection::establish_connection();

    rocket::build()
        .manage(pool)
        .mount("/", routes![server::server_router::health])
        .mount(
            "/users",
            routes![
                users::user_router::get_users,
                users::user_router::user_detail
            ],
        )
}
