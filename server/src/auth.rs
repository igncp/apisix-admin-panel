use actix_web::{
    cookie::{time::Duration, Cookie},
    post, web, HttpResponse,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{
    config::{HandlerConfig, User},
    server_error::{CommonResponse, RespError},
};

#[derive(Debug, Serialize, Deserialize)]
struct TokenClaims {
    exp: usize,
    sub: String,
}

#[post("/login")]
async fn login(config: HandlerConfig, body: web::Json<User>) -> CommonResponse {
    if config.jwt_secret.is_empty() {
        return Ok(HttpResponse::Ok().body("OK"));
    }

    let found_user = config
        .users
        .iter()
        .find(|u| u.username == body.username && u.password == body.password)
        .ok_or(RespError::Unauthorized)?;

    let my_claims = TokenClaims {
        exp: 9999999999,
        sub: found_user.username.clone(),
    };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )
    .map_err(|_| RespError::Unauthorized)?;

    let c = Cookie::build("token", token)
        .path("/")
        .max_age(Duration::new(60 * 60 * 24 * 30, 0))
        .finish();

    Ok(HttpResponse::Ok().cookie(c).body("OK"))
}

#[post("/logout")]
async fn logout() -> CommonResponse {
    let c = Cookie::build("token", "")
        .path("/")
        .secure(false)
        .http_only(true)
        .finish();

    Ok(HttpResponse::Ok().cookie(c).body("OK"))
}

pub fn get_auth_scope() -> actix_web::Scope {
    web::scope("/auth").service(login).service(logout)
}

pub fn verify_auth(req: &actix_web::HttpRequest, config: &HandlerConfig) -> Result<(), RespError> {
    if config.jwt_secret.is_empty() {
        return Ok(());
    }

    let token = req.cookie("token").ok_or(RespError::Unauthorized)?;
    let token = token.value();

    decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| RespError::Unauthorized)?;

    Ok(())
}
