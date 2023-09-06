use std::error::Error;

use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};

const SALT: &str = "wbKrGKFAzQf83ZXa";

pub fn hash(value: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let salt = SaltString::from_b64(SALT).unwrap();
    //TODO
    let argon = Argon2::default();
    match argon.hash_password(value, &salt) {
        //TODO
        Ok(res) => Ok(res.hash.unwrap().as_bytes().to_vec()),
        Err(_) => Err("Something went wrong".into()),
    }
}
