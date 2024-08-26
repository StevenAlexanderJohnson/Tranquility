use data_access::AuthUser;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

pub fn verify_token(token: &str) -> Result<BTreeMap<String, String>, Box<dyn std::error::Error>> {
    let key_string = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key: Hmac<Sha256> = Hmac::new_from_slice(key_string.as_bytes()).unwrap();

    let claims: Result<BTreeMap<String, String>, _> = token.verify_with_key(&key);
    Ok(claims?)
}

pub fn generate_token(auth_user: &AuthUser) -> Result<String, Box<dyn std::error::Error>> {
    let key_string = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key: Hmac<Sha256> = Hmac::new_from_slice(key_string.as_bytes()).unwrap();

    let mut claims = BTreeMap::new();
    claims.insert("username".to_string(), auth_user.username.clone());
    claims.insert("exp".to_string(), "3600".to_string());
    claims.insert("id".to_string(), auth_user.id.as_ref().unwrap().to_string());

    let token = claims.sign_with_key(&key).unwrap();

    Ok(token)
}
