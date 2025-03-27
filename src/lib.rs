use wasm_bindgen::prelude::wasm_bindgen;
use worker::*;

use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};

#[wasm_bindgen]
pub async fn hash(password: String, salt: String) -> String {
    console_error_panic_hook::set_once();

    let password = password.as_bytes();

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let salt = match SaltString::from_b64(&salt) {
        Ok(salt) => salt,
        Err(error) => {
            console_log!("Error decoding salt: {}", error);
            return "Decoding failed".to_string();
        }
    };

    match argon2.hash_password(password, &salt) {
        Ok(hash) => hash.to_string(),
        Err(error) => {
            console_log!("Error hashing password: {}", error);
            "Hashing failed".to_string()
        }
    }
}
