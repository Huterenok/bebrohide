use anyhow::{Context, Result};

use serde::{Deserialize, Serialize};

pub fn serialize<T: Serialize>(data: &T) -> Result<Vec<u8>> {
    return bincode::serialize(data).with_context(|| format!("Something went wrong"));
}

pub fn deserialize<'de, T: Deserialize<'de>>(data: &'de [u8]) -> Result<T> {
    return bincode::deserialize(data).with_context(|| format!("Something went wrong"));
}
