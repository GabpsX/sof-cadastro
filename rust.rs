use rust_crypto::symmetriccipher::SymmetricCipherError;
use rust_crypto::aead::{AeadEncryptor, NewAead};
use rust_crypto::aes::Aes256Gcm;
use rust_crypto::util::fixed_time_eq;

use rand::rngs::OsRng;
use rand::RngCore;

#[tauri::command]
fn encryption_nonce() -> Result<serde_json::Value, SymmetricCipherError> {
    let mut key = vec![0u8; 32];
    OsRng.fill_bytes(&mut key);

    let cipher = Aes256Gcm::new(&key.into());

    let mut nonce = vec![0u8; 12];
    OsRng.fill_bytes(&mut nonce);

    let message = b"Example message";
    let ciphertext = cipher.encrypt(&nonce.into(), message.as_ref())?;

    let key_str = base64::encode(&key);
    let token_str = base64::encode(&ciphertext);

    let result = serde_json::json!({
        "key": key_str,
        "token": token_str
    });

    Ok(result)
}
