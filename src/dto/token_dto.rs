use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenReadDto {
    pub token: String,
    pub iat: i64,
    pub exp: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenClaimsDto {
    pub sub: i32,
    pub email: String,
    pub iat: i64,
    pub exp: i64,
}
