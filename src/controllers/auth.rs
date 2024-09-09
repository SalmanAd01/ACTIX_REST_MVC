use actix_session::Session;
use actix_web::{web::{self}, HttpResponse, Responder};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{db::connect::DbPool, models::user::{NewUser, User}, schema::{self, users::username}};
use super::super::schema::users;

#[derive(Deserialize, Serialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

pub async fn signup(
    pool: web::Data<DbPool>,
    data: web::Json<NewUser>,
) -> impl Responder {
    let hash_password = crate::utils::bcrypt::hash_password(&data.password).expect("Failed to hash password");
    let new_user = NewUser {
        name: data.name.clone(),
        email: data.email.clone(),
        username: data.username.clone(),
        password: hash_password,
    };

    let mut db_connection = pool.get().expect("Failed to get DB connection from pool");

    match web::block(move || {
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&mut db_connection)
    }).await {
        Ok(_) => HttpResponse::Created().finish(), 
        Err(e) => {
            eprintln!("Error inserting new user: {:?}", e);
            HttpResponse::InternalServerError().finish() 
        }
    }
}

pub async fn login(
    pool: web::Data<DbPool>,
    data: web::Json<LoginData>,
    session: Session,
) -> impl Responder {
    let mut db_connection = pool.get().expect("Failed to get DB connection from pool");

    let get_user_result = schema::users::table
    .filter(username.eq(&data.username))
    .first::<User>(&mut db_connection);

    match get_user_result {
        Ok(user) => {
            let is_valid = crate::utils::bcrypt::verify_password(&data.password, &user.password).expect("Failed to verify password");
            if is_valid {
                let jwt_token = crate::utils::jwt::encode_user(&user).expect("Failed to encode user");
                session.insert("jwt_token", jwt_token).expect("Failed to set session");
                session.renew();
                HttpResponse::Ok().json(json!({
                    "message": "Login successful",
                    "user": user
                }))
            } else {
                HttpResponse::Unauthorized().finish()
            }
        },
        Err(e) => {
            eprintln!("Error logging in: {:?}", e);
            HttpResponse::Unauthorized().finish()
        }
    }
}

