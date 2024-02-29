use rocket::serde::json::Json;
use crate::utils;

#[get("/health")]
pub fn health() -> Json<utils::common::Response<String>> {
    let resp = utils::common::Response {
        message: String::from("server is up"),
        ok: bool::from(true),
        data: None,
    };

    return Json(resp);
}
