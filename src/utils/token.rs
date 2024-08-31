use std::env;

use jsonwebtoken::{errors::Error, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constants::env_key;

static ONE_DAY: i64 = 60 * 60 * 24; // in seconds

fn get_secret_key() -> String {
    env::var(env_key::SECRET_KEY).expect("SECREET_KEY must be set")
}
pub fn generate(user_id: Uuid, now: i64) -> Result<String, Error> {
    let claims = Claims::new(user_id, now);
    let binding = get_secret_key();
    let secret_key = binding.as_bytes();
    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key),
    )
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: i64,
    iat: i64,
    user_id: Uuid,
}

impl Claims {
    pub fn new(user_id: Uuid, now: i64) -> Self {
        Claims {
            exp: now + ONE_DAY,
            iat: now,
            user_id,
        }
    }
}
