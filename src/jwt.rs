use crate::handler::User;
use axum::http::{header::AUTHORIZATION, HeaderMap};
use jsonwebtoken::{
    decode, encode, errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header,
    TokenData, Validation,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

// should be in .env
pub const JWT_SECRET_KEY: &str = "app-secret";
pub const _JWT_HEADER_KEY: &str = "Authorization";
pub const _JWT_COOKIE_KEY: &str = "Authorization";

// build Claims
pub trait ClaimsGenerator<T> {
    fn generate_claims(_: &T) -> Self;
}

// decode token
pub trait JwtDecoder<T: DeserializeOwned> {
    fn parse_header(request: &HeaderMap) -> Result<String, String>;
    // check token and decode
    fn decode(&self, token: &str) -> Result<TokenData<T>, JwtError>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiClaims {
    iat: i64,          // issued at
    exp: i64,          // expiration
    sub: String,       // subject
    user_name: String, // user_name
}

impl ClaimsGenerator<User> for ApiClaims {
    fn generate_claims(user: &User) -> Self {
        let now = chrono::Utc::now().timestamp();
        let exp = now + 60 * 60 * 24 * 7; // 7 days
        ApiClaims {
            iat: now,
            exp,
            sub: "auth".to_string(),
            user_name: user.name.clone(),
        }
    }
}

pub struct ApiJwt;

impl ApiJwt {
    pub fn encode<T: Serialize>(claims: T) -> Result<String, JwtError> {
        let header = Header {
            typ: Some("JWT".into()),
            alg: Algorithm::HS256,
            ..Default::default()
        };

        encode(
            &header,
            &claims,
            &EncodingKey::from_secret(JWT_SECRET_KEY.as_ref()),
        )
    }
}

impl JwtDecoder<ApiClaims> for ApiJwt {
    fn parse_header(header: &HeaderMap) -> Result<String, String> {
        match header.get(AUTHORIZATION) {
            Some(token) => {
                let mut split_token = token
                    .to_str()
                    .map_err(|_| "Invalid header")?
                    .split_whitespace();
                match split_token.next() {
                    Some(schema_type) if schema_type == "Bearer" => match split_token.next() {
                        Some(jwt_token) => Ok(jwt_token.to_string()),
                        None => Err("No JWT token found".to_string()),
                    },
                    Some(_) | None => Err("Invalid schema type".to_string()),
                }
            }
            None => Err("No Authorization header found".to_string()),
        }
    }

    fn decode(&self, token: &str) -> Result<TokenData<ApiClaims>, JwtError> {
        decode::<ApiClaims>(
            token,
            &DecodingKey::from_secret(JWT_SECRET_KEY.as_ref()),
            &Validation::default(),
        )
    }
}
