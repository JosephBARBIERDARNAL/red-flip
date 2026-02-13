use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // user id
    pub exp: usize,
    pub iat: usize,
}

pub fn create_token(user_id: &str, secret: &str) -> Result<String, AppError> {
    let now = Utc::now();
    let claims = Claims {
        sub: user_id.to_string(),
        iat: now.timestamp() as usize,
        exp: (now + Duration::days(7)).timestamp() as usize,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;
    Ok(token)
}

pub fn validate_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn create_and_validate_token_round_trip() {
        let user_id = "user-123";
        let secret = "test-secret";

        let token = create_token(user_id, secret).expect("token should be created");
        let claims = validate_token(&token, secret).expect("token should validate");

        assert_eq!(claims.sub, user_id);
        assert!(claims.exp > claims.iat);

        let ttl = claims.exp as i64 - claims.iat as i64;
        assert!((6 * 24 * 60 * 60..=8 * 24 * 60 * 60).contains(&ttl));
        assert!(claims.iat as i64 <= Utc::now().timestamp());
    }

    #[test]
    fn validate_token_rejects_wrong_secret() {
        let token = create_token("user-123", "right-secret").expect("token should be created");

        let result = validate_token(&token, "wrong-secret");

        assert!(matches!(result, Err(AppError::Unauthorized(_))));
    }
}
