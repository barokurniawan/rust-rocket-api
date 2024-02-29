use serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserAccount {
    pub username: String,
    pub email: String,
}
