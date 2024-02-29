use diesel::{deserialize::Queryable, Selectable};
use serde::Serialize;
use crate::base_schema::users;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserModel {
    pub id: i32,
    pub email: Option<String>,
    pub username: Option<String>,
}