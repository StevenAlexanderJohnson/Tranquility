use data_access::AuthUser;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub username: String,
    #[serde(rename = "exp")]
    pub expiration: u64,
    pub id: i32,
}

pub fn verify_token(token: &str) -> Result<Claims, Box<dyn std::error::Error>> {
    let key_string = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key: Hmac<Sha256> = Hmac::new_from_slice(key_string.as_bytes()).unwrap();

    let claims: Result<Claims, _> = token.verify_with_key(&key);
    Ok(claims?)
}

pub fn generate_token(auth_user: &AuthUser) -> Result<String, Box<dyn std::error::Error>> {
    let key_string = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key: Hmac<Sha256> = Hmac::new_from_slice(key_string.as_bytes()).unwrap();

    let lifespan = std::time::Duration::from_secs(120);
    let claims = Claims {
        username: auth_user.username.clone(),
        expiration: lifespan.as_secs(),
        id: *auth_user.id.as_ref().unwrap(),
    };

    let token = claims.sign_with_key(&key).unwrap();

    Ok(token)
}
