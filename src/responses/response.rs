use serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response<T> {
    pub message: String,
    pub ok: bool,
    pub data: Option<T>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PagingResponse<T> {
    pub data: Option<T>,
    pub current_page: i32,
    pub has_next_page: bool,
    pub last_page: i32,
}