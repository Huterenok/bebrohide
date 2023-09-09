use anyhow::{bail, Result};

use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};

const SALT: &str = "wbKrGKFAzQf83ZXa";

pub fn hash(value: &[u8]) -> Result<Vec<u8>> {
    let salt = SaltString::from_b64(SALT).unwrap();
    let argon = Argon2::default();

    match argon.hash_password(value, &salt) {
        Ok(res) => Ok(res.hash.unwrap().as_bytes().to_vec()),
        Err(_) => bail!("Something went wrong"),
    }
}
