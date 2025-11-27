use crate::math::u256::U256;

use sha2::{Digest, Sha256};

pub fn sha256_bytes(input: &[u8]) -> U256 {
    let mut hasher = Sha256::new();
    hasher.update(input);

    let digest: [u8; 32] = hasher.finalize().into();

    digest.into()
}
