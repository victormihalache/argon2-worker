use wasm_bindgen::prelude::wasm_bindgen;
use worker::*;

use argon2::*;

#[wasm_bindgen]
pub async fn hash(password: String, salt: String) -> String {
    console_error_panic_hook::set_once();

    let config = Config::default();

    let hash: String = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap();

    return hash.to_string();
}
