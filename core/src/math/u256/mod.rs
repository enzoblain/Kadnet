use core::fmt;

use super::{ArithmeticError, OperationResult};

use rand::RngCore;

pub mod conv;
pub mod ops;

#[derive(Copy, Clone, Debug, Default)]
pub struct U256(pub [u8; 32]);
impl U256 {
    pub const MAX: Self = Self([255u8; 32]);

    pub fn generate_random() -> Self {
        let mut id = Self::default();

        rand::rng().fill_bytes(&mut id.0);

        id
    }

    pub fn two_pow_k(k: u8) -> Self {
        let mut out = [0u8; 32];
        let byte_index = 31 - (k / 8);
        let bit_index = k % 8;

        out[byte_index as usize] = 1 << bit_index;

        Self::from(out)
    }
}

impl fmt::Display for U256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, b) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str(":")?;
            }
            write!(f, "{:02X}", b)?;
        }

        Ok(())
    }
}
