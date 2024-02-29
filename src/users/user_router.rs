use rocket::serde::json::Json;
use crate::utils::common::{PagingResponse, Response};
use crate::users::entities::UserAccount;

#[get("/")]
pub fn get_users() -> Json<Response<PagingResponse<Vec<UserAccount>>>> {
    let mut users: Vec<UserAccount> = Vec::new();
    let mut i = 0;
    while i <= 25 {
        let username = format!("user-{}", i);
        let email = format!("email-{}", i);

        users.push(UserAccount {
            username: username,
            email: email,
        });
        i += 1;
    }

    let resp = Response {
        message: String::from("this is a list users API"),
        ok: bool::from(true),
        data: Some(PagingResponse {
            data: Some(users),
            current_page: 1,
            last_page: 10,
            has_next_page: true,
        })
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
