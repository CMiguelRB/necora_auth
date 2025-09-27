use aes_gcm::{
    aead::{consts::U12, generic_array::GenericArray, Aead, OsRng}, AeadCore, Aes256Gcm, Key, KeyInit
};
use base64::{engine::general_purpose, Engine};
use std::io::Error;
use crate::conf::config;

pub fn encrypt(value: &str) -> Result<String, Error> {
    let config = config::settings();
    let key: &String = &config.security.encryption_key;
    let password_byte = key.as_bytes();

    let key: &Key<Aes256Gcm> = password_byte.into();
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let encrypted_bytes = cipher.encrypt(&nonce, value.as_bytes());

    let mut hash = Vec::new();
    hash.extend_from_slice(&nonce);
    hash.extend_from_slice(&encrypted_bytes.unwrap());

    let encrypted = general_purpose::STANDARD.encode(hash);

    return Ok(encrypted);
}

pub fn decrypt(value: &str) -> Result<String, Error> {
    let config = config::settings();
    let key: &String = &config.security.encryption_key;
    let password_byte = key.as_bytes();

    let hash = general_purpose::STANDARD.decode(value).expect("invalid base64");
    
    // nonce: first 12 bytes, rest: ciphertext
    let (nonce_bytes, ciphertext) = hash.split_at(12);
    let nonce: &aes_gcm::aead::generic_array::GenericArray<u8, U12> = GenericArray::from_slice(nonce_bytes);

    let key: &Key<Aes256Gcm> = password_byte.into();

    let nonce = aes_gcm::Nonce::from_slice(&nonce);

    let cipher = Aes256Gcm::new(&key);

    let decrypted = cipher.decrypt(nonce, ciphertext).expect("decryption failed");
    let decrypted_string = String::from_utf8(decrypted).expect("invalid utf-8");
    
    return Ok(decrypted_string);
}
