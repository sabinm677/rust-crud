use chrono::{DateTime, Duration, TimeDelta, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, encode, errors::Error};
use serde::{Deserialize, Serialize};

use crate::utils::constants;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
    pub id: i32
}

pub fn encode_jwt(email: String, id: i32) -> Result<String, Error> {
    let now: DateTime<Utc> = Utc::now();
    let expire: TimeDelta = Duration::hours(24);
    let claims = Claims {
        exp: now.timestamp() as usize + 3600,
        iat: (now+expire).timestamp() as usize,
        email,
        id,
    };
    let secret = (*constants::SECRET).clone();

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}


pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>, Error> {
    let secret = (*constants::SECRET).clone();

    let claim_data: Result<TokenData<Claims>, Error> = jsonwebtoken::decode::<Claims>(&jwt, &DecodingKey::from_secret(secret.as_ref()), &Validation::default());

    claim_data
}