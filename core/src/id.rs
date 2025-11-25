use core::mem::transmute;
use core::ops::{Add, BitXor, Sub};
use rand::RngCore;

#[derive(Copy, Clone, Debug, Default)]
pub struct U256(pub [u8; 32]);

impl<'a> BitXor<&'a U256> for &U256 {
    type Output = U256;

    fn bitxor(self, dest: &'a U256) -> Self::Output {
        let left: &[u128; 2] = unsafe { transmute(&self) };
        let right: &[u128; 2] = unsafe { transmute(&dest) };

        let distance: [u128; 2] = std::array::from_fn(|i| left[i] ^ right[i]);

        U256::from(distance)
    }
}

impl Add for U256 {
    type Output = U256;

    fn add(self, other: U256) -> Self::Output {
        let mut out = [0u8; 32];
        let mut carry = 0u16;

        for i in (0..32).rev() {
            let sum = self.0[i] as u16 + other.0[i] as u16 + carry;
            out[i] = (sum & 0xFF) as u8;
            carry = sum >> 8;
        }

        U256(out)
    }
}

impl Sub for U256 {
    type Output = U256;

    fn sub(self, other: U256) -> Self::Output {
        let mut out = [0u8; 32];
        let mut borrow = 0u16;

        for i in (0..32).rev() {
            let lhs = self.0[i] as i16;
            let rhs = other.0[i] as i16 + borrow as i16;
            if lhs >= rhs {
                out[i] = (lhs - rhs) as u8;
                borrow = 0;
            } else {
                out[i] = ((lhs + 256) - rhs) as u8;
                borrow = 1;
            }
        }

        U256(out)
    }
}

impl U256 {
    pub const MAX: Self = Self([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    ]);

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

impl From<[u128; 2]> for U256 {
    fn from(value: [u128; 2]) -> Self {
        let mut out = [0u8; 32];

        out[..16].copy_from_slice(&value[0].to_be_bytes());
        out[16..].copy_from_slice(&value[1].to_be_bytes());

        Self(out)
    }
}

impl From<[u8; 32]> for U256 {
    fn from(value: [u8; 32]) -> Self {
        Self(value)
    }
}

impl From<u32> for U256 {
    fn from(value: u32) -> Self {
        let mut out = [0u8; 32];

        out[28..32].copy_from_slice(&value.to_be_bytes());

        U256(out)
    }
}
