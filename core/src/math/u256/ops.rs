use super::{ArithmeticError, OperationResult, U256};

use core::array;
use core::ops::{Add, BitXor, Sub};

impl BitXor<U256> for U256 {
    type Output = U256;

    fn bitxor(self, rhs: U256) -> Self::Output {
        let out = array::from_fn(|i| self.0[i] ^ rhs.0[i]);

        U256::from(out)
    }
}

impl Add for U256 {
    type Output = OperationResult<U256>;

    fn add(self, rhs: U256) -> Self::Output {
        let mut out = [0u8; 32];
        let mut carry = 0u16;

        for i in (0..32).rev() {
            let sum = self.0[i] as u16 + rhs.0[i] as u16 + carry;
            out[i] = (sum & 0xFF) as u8;
            carry = sum >> 8;
        }

        if carry != 0 {
            return OperationResult::Bounds(ArithmeticError::Overflow(U256(out)));
        }

        OperationResult::Ok(U256(out))
    }
}

impl Sub for U256 {
    type Output = OperationResult<U256>;

    fn sub(self, rhs: U256) -> Self::Output {
        let mut out = [0u8; 32];
        let mut borrow = 0u16;

        for i in (0..32).rev() {
            let lhs = self.0[i] as i16;
            let rhs = rhs.0[i] as i16 + borrow as i16;

            if lhs >= rhs {
                out[i] = (lhs - rhs) as u8;
                borrow = 0;
            } else {
                out[i] = ((lhs + 256) - rhs) as u8;
                borrow = 1;
            }
        }

        if borrow == 1 {
            return OperationResult::Bounds(ArithmeticError::UnderFlow(U256(out)));
        }

        OperationResult::Ok(U256(out))
    }
}
