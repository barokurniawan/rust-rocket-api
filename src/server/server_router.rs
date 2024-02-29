use crate::responses::response::Response;
use rocket::serde::json::Json;

#[get("/health")]
pub fn health() -> Json<Response<String>> {
    let resp = Response {
        message: String::from("server is up"),
        ok: bool::from(true),
        data: None,
    };

    return Json(resp);
}
