use diesel::prelude::{Insertable, Queryable};
use serde::{Serialize, Deserialize};
use crate::schema::users;

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser{
    pub name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct User{
    pub id: i32,
    pub name: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}