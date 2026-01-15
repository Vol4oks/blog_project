use argon2::{
    Argon2, PasswordVerifier,
    password_hash::{PasswordHash, PasswordHasher, SaltString, rand_core::OsRng},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Clone)]
pub struct JwtService {
    secret: String,
    expiration_minuts: i64,
}

impl JwtService {
    pub fn new(secret: String, expiration_minuts: i64) -> Self {
        Self {
            secret,
            expiration_minuts,
        }
    }

    pub fn generate_token(&self, user_id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = Claims {
            sub: user_id.to_string(),
            exp: chrono::Utc::now()
                .checked_add_signed(chrono::Duration::minutes(self.expiration_minuts))
                .unwrap()
                .timestamp() as usize,
            iat: chrono::Utc::now().timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )?;
        Ok(data.claims)
    }
}

pub fn password_hash(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

pub fn password_verify(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let pasrsed = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(password.as_bytes(), &pasrsed)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_service() {
        let jwt_service = JwtService::new("secret".to_string(), 24 * 60);
        let user_id = Uuid::new_v4();

        let token = jwt_service
            .generate_token(user_id)
            .expect("Failed to generate token");
        dbg!(&token);

        let claims = jwt_service
            .verify_token(&token)
            .expect("Failed to verify token");

        assert_eq!(claims.sub, user_id.to_string());
    }

    #[test]
    fn test_jwt_service_invalid_token() {
        let jwt_service = JwtService::new("secret".to_string(), 24 * 60);

        let other_service = JwtService::new("different_secret".to_string(), 24 * 60);
        let token = other_service
            .generate_token(Uuid::new_v4())
            .expect("Failed to generate token");

        let result = jwt_service.verify_token(&token);

        assert!(result.is_err());
    }
}
