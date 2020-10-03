use crate::errors::ServiceError;
use argon2::{self, Config};

lazy_static::lazy_static! {
pub static ref SECRET_KEY: String = std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
}

pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    let password = password.as_bytes();
    let salt = SECRET_KEY.as_bytes();
    let config = Config::default();
    argon2::hash_encoded(password, salt, &config).map_err(|err| {
        dbg!(err);
        ServiceError::InternalServerError
    })
}

pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
    argon2::verify_encoded(hash, password.as_bytes())
    .map_err(|err| {
        dbg!(err);
        ServiceError::Unauthorized
    })
}
