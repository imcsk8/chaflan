/// Authentication implementation
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation,
};
use lazy_static::lazy_static;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    response::status::Custom,
};
use serde::{Deserialize, Serialize};

const BEARER: &str = "Bearer";
const AUTHORIZATION: &str = "Authorization";

///Key for symmetric token encoding
const SECRET: &str = "TestSecret!";

lazy_static! {
    /// Time for token expiration
    //TODO: check if we can register this on rocket build
    static ref TOKEN_DURATION: Duration = Duration::minutes(5);
}

/// Manage authentication decoding errors
#[derive(Debug, PartialEq)]
pub enum AuthenticationError {
    Missing,
    Decoding(String),
    Expired,
}

/// jsonwebtoken Claim

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub name: String,
    exp: usize,
}

//Rocket request guard
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = AuthenticationError;
    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let token_str = if let Some(cookie) = request.cookies().get("jwt") {
            Some(cookie.value().to_string())
        } else if let Some(header) = request.headers().get_one(AUTHORIZATION) {
            header.strip_prefix(BEARER).map(str::trim).map(String::from)
        } else {
            None
        };

        match token_str {
            Some(t) => match Claims::from_authorization(&t) {
                Ok(c) => Outcome::Success(c),
                Err(e) => Outcome::Error((Status::Forbidden, e)),
            },
            None => Outcome::Error((Status::Forbidden, AuthenticationError::Missing)),
        }
    }
}

/// Claims implementation
impl Claims {
    /// Creates a new claim with a given name
    pub fn from_name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            exp: 0,
        }
    }

    /// Create Claims from a token string
    fn from_authorization(token: &str) -> Result<Self, AuthenticationError> {
        // Get claims from a JWT
        let token = decode::<Claims>(
            token,
            &DecodingKey::from_secret(SECRET.as_ref()),
            &Validation::default(), //TODO check this defaults
        )
        .map_err(|e| match e.kind() {
            ErrorKind::ExpiredSignature => AuthenticationError::Expired,
            //TODO: check if we have different responses for each error
            _ => AuthenticationError::Decoding(e.to_string()),
        })?;

        Ok(token.claims)
    }

    /// Convert this Claims instance to a token string to be sent to the browser
    pub fn into_token(mut self) -> Result<String, Custom<String>> {
        let expiration = Utc::now()
            .checked_add_signed(*TOKEN_DURATION)
            .expect("Failed to create expiration time")
            .timestamp();
        self.exp = expiration as usize;

        // Create the JWT
        let token = encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(SECRET.as_ref()),
        )
        .map_err(|e| Custom(Status::BadRequest, e.to_string()))?;

        Ok(token)
    }
}
