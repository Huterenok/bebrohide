use serde::{Deserialize, Serialize};

pub const NONCE_SIZE: usize = 12;
pub const VISIBLE_MASK_SIZE: usize = 4 + NONCE_SIZE + std::mem::size_of::<u64>() * 3;
pub const MAGIC_BYTES: [u8; 2] = [77, 77];

#[derive(Serialize, Deserialize)]
pub struct HiddenMask {
    pub name: String,
}

impl HiddenMask {
    pub fn new(name: String) -> Self {
        HiddenMask { name }
    }

    pub fn size(&self) -> u64 {
        return self.name.len() as u64 + 8;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VisibleMask {
    pub magic_bytes1: [u8; 2],
    pub encoded_hidden_length: u64,
    pub hidden_file_length: u64,
    pub hidden_mask_length: u64,
    pub nonce: [u8; 12],
    pub magic_bytes2: [u8; 2],
}

impl VisibleMask {
    pub fn new(
        encoded_hidden_length: u64,
        hidden_file_length: u64,
        hidden_mask_length: u64,
        nonce: [u8; 12],
    ) -> Self {
        Self {
            magic_bytes1: MAGIC_BYTES,
            encoded_hidden_length,
            hidden_file_length,
            hidden_mask_length,
            nonce,
            magic_bytes2: MAGIC_BYTES,
        }
    }

    pub fn verify(&self) -> bool {
        self.magic_bytes1 == MAGIC_BYTES && self.magic_bytes2 == MAGIC_BYTES
    }
}
