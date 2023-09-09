use anyhow::{bail, Result};

use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{AeadCore, Aes256Gcm, Key, KeyInit};

pub fn encode(pwd: &[u8], data: &[u8]) -> Result<(Vec<u8>, [u8; 12])> {
    let key = Key::<Aes256Gcm>::from_slice(pwd);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    match cipher.encrypt(&nonce, data) {
        Ok(encoded) => Ok((encoded, nonce.try_into().unwrap())),
        Err(_) => bail!("Something went wrong while encoding"),
    }
}

pub fn decode(pwd: &[u8], nonce: &[u8; 12], data: &[u8]) -> Result<Vec<u8>> {
    let key = Key::<Aes256Gcm>::from_slice(pwd);
    let cipher = Aes256Gcm::new(&key);
    let nonce = GenericArray::from_slice(nonce);

    match cipher.decrypt(nonce, data) {
        Ok(decoded) => Ok(decoded),
        Err(_) => bail!("Password is incorrect"),
    }
}
