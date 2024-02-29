use crate::base_schema::users::dsl::users;
use crate::libraries::db_connection::{establish_connection, MysqlPool};
use crate::responses::response::{PagingResponse, Response};
use crate::users::entities::UserAccount;
use diesel::RunQueryDsl;
use rocket::State;
use rocket::{get, serde::json::Json};

use super::models::UserModel;

#[get("/")]
pub fn get_users(pool: &State<MysqlPool>) -> Json<Response<PagingResponse<Vec<UserModel>>>> {
    let conn = &mut pool.get().expect("Failed to acquire connection from pool");
    let row_users = users.load::<UserModel>(conn).expect("error apa nich");

    let resp = Response {
        message: String::from("this is a list users API"),
        ok: bool::from(true),
        data: Some(PagingResponse {
            data: Some(row_users),
            current_page: 1,
            last_page: 10,
            has_next_page: true,
        }),
    };

    return Json(resp);
}

#[get("/detail/<username>")]
pub fn user_detail(username: &str) -> Json<Response<UserAccount>> {
    let resp = Response {
        message: "".to_string(),
        ok: bool::from(true),
        data: Some(UserAccount {
            email: username.to_string(),
            username: username.to_string(),
        }),
    };

    return Json(resp);
}
