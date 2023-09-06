use std::error::Error;

use serde::{Deserialize, Serialize};

pub fn serialize<T: Serialize>(data: &T) -> Result<Vec<u8>, Box<dyn Error>> {
    match bincode::serialize(data) {
        Ok(data) => Ok(data),
        Err(_) => Err("Something went wrong".into()),
    }
}

pub fn deserialize<'de, T: Deserialize<'de>>(data: &'de [u8]) -> Result<T, Box<dyn Error>> {
    match bincode::deserialize(data) {
        Ok(mask) => Ok(mask),
        Err(_) => {
            Err("Something went wrong".into())
        }
    }
}
