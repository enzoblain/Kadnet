use super::entry::Entries;
use crate::{K, N_BUCKETS, U256};

use core::array;
use core::net::IpAddr;
use core::ops::RangeInclusive;

pub enum BucketError {
    BucketFull,
}

pub struct Bucket {
    pub range: RangeInclusive<U256>,
    pub max_size: usize,
    pub size: usize,
    pub value: Entries,
}

impl Bucket {
    pub fn init(x: U256, b: u8) -> Self {
        let bottom_value = ((x >> (U256::from(b) + U256::from(1u32)))
            << (U256::from(b) + U256::from(1u32)))
            + ((U256::from(1u32) - ((x >> b.into()) & U256::from(1u32))) << U256::from(b));
        let top_value = bottom_value + U256::two_pow_k(2) - 1u32.into();

        let range = top_value - bottom_value;
        let max_size = if let Ok(k) = range.try_into() { k } else { K };

        Self {
            range: bottom_value..=top_value,
            max_size: max_size as usize,
            size: 0,
            value: Entries::default(),
        }
    }

    pub fn init_buckets(x: U256) -> [Self; N_BUCKETS] {
        array::from_fn(|i| Bucket::init(x, i as u8))
    }

    pub fn is_full(&self) -> bool {
        self.max_size == self.size
    }

    pub fn add(&mut self, addr: IpAddr) -> Result<(), BucketError> {
        if self.is_full() {
            return Err(BucketError::BucketFull);
        }

        self.value.add_entry(addr, self.size);
        self.size += 1;

        Ok(())
    }
}
