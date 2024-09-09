
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::models::user::User;
use jsonwebtoken::{encode, errors::Error, DecodingKey, EncodingKey, Header};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: String,
    exp: i64,
}

pub fn encode_user(user: &User) -> Result<String, Error> {
    let secret = "fsdjfhsdfjhkdsfjhsdkjf";
    let max_age = 60 * 60 * 24;
    let now = Utc::now().timestamp_nanos_opt().unwrap_or(0) / 1_000_000_000;

    let claims = Claims {
        user_id: user.id.to_string(),
        exp: now + max_age,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;
    Ok(token)
}

pub fn decode_user(token: String) -> Result<String, Error> {
    let secret = "fsdjfhsdfjhkdsfjhsdkjf";
    let token_data = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )?;

    let user_id = token_data.claims.user_id.parse().unwrap();
    Ok(user_id)
}
