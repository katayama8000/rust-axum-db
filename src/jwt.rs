use axum::{
    extract::FromRequest,
    http::{self, request},
};
use jsonwebtoken::{DecodingKey, EncodingKey, TokenData};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::AppState;

pub const JWT_SECRET_KEY: &str = "app-secret";
pub const JWT_HEADER_KEY: &str = "Authorization";
pub const JWT_COOKIE_KEY: &str = "Authorization";
// build Claims
pub trait ClaimsGenerator<T> {
    fn generate_claims(_: &T) -> Self;
}

// encode token
pub trait JwtEncoder {
    fn encode<T: Serialize>(claims: T) -> String {
        let mut header = jsonwebtoken::Header::default();
        header.typ = Some(String::from("JWT"));
        header.alg = jsonwebtoken::Algorithm::HS256;
        // make token with claims and header
        jsonwebtoken::encode(
            &header,
            &claims,
            &EncodingKey::from_secret(JWT_SECRET_KEY.as_ref()),
        )
        .unwrap()
    }
}

// decode token
pub trait JwtDecoder<T: DeserializeOwned, E, R> {
    fn parse_header(&self, request: &R) -> Result<String, E>;
    // check token and decode
    fn decode(&self, token: &str) -> Result<TokenData<T>, jsonwebtoken::errors::Error> {
        match jsonwebtoken::decode::<T>(
            token,
            &DecodingKey::from_secret(JWT_SECRET_KEY.as_ref()),
            &jsonwebtoken::Validation::default(),
        ) {
            Ok(token_data) => Ok(token_data),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]

pub struct ApiClaims {
    iat: i64,          // issued at
    exp: i64,          // expiration
    sub: String,       // subject
    user_id: i32,      // user_id
    user_name: String, // user_name
}

pub struct UserDto {
    pub user_id: i32,
    pub user_name: String,
    pub password: String,
}

impl ClaimsGenerator<UserDto> for ApiClaims {
    fn generate_claims(user: &UserDto) -> Self {
        let now = chrono::Utc::now().timestamp();
        let exp = now + 60 * 60 * 24 * 7; // 7 days
        ApiClaims {
            iat: now,
            exp,
            sub: "auth".to_string(),
            user_id: user.user_id,
            user_name: user.user_name.clone(),
        }
    }
}

#[derive(Default)]
pub struct ApiJwt;
impl JwtEncoder for ApiJwt {}
impl JwtDecoder<ApiClaims, String, http::Request<String>> for ApiJwt {
    fn parse_header(&self, request: &http::Request<String>) -> Result<String, String> {
        // get token from header
        let header_value = match request.headers().get(JWT_HEADER_KEY) {
            Some(token) => Ok(token.to_str().unwrap().to_string()),
            None => Err("No token found".to_string()),
        };

        let token = header_value.unwrap();
        let mut split_token = token.split_whitespace();
        match split_token.next() {
            Some(schema_type) => {
                if schema_type != "Bearer" {
                    return Err("Invalid schema type".to_string());
                }
            }
            None => return Err("No schema type found".to_string()),
        };

        match split_token.next() {
            Some(jwt_token) => Ok(jwt_token.to_string()),
            None => Err("No JWT token found".to_string()),
        }
    }
}

impl ApiClaims {
    pub fn from_request(req: &http::Request<String>) -> Result<Self, http::StatusCode> {
        let request = req.clone();
        let jwt = ApiJwt::default();
        let token = jwt.parse_header(&request).unwrap();
        match jwt.decode(&token) {
            Ok(token_data) => Ok(token_data.claims),
            Err(_) => Err(http::StatusCode::UNAUTHORIZED),
        }
    }
}
