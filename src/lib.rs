use wasm_bindgen::prelude::wasm_bindgen;
use worker::*;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

#[wasm_bindgen]
pub async fn hash(password: String) -> String {
    console_error_panic_hook::set_once();

    let password = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2.hash_password(password, &salt)?.to_string();

    return password_hash;
}
