use crate::{ALPHA, K, N_BUCKETS, U256};

use core::array;
use core::net::{IpAddr, Ipv4Addr};
use core::ops::RangeInclusive;

pub enum BucketError {
    BucketFull,
}

#[derive(Debug)]
pub struct Bucket {
    pub range: RangeInclusive<U256>,
    pub max_size: u8,
    pub size: u8,
    pub value: [(U256, IpAddr); K as usize], // Oldest -> Newest
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
            max_size,
            size: 0,
            value: [(U256::default(), IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))); K as usize],
        }
    }

    pub fn init_buckets(x: U256) -> [Self; N_BUCKETS] {
        array::from_fn(|i| Bucket::init(x, i as u8))
    }

    pub fn is_full(&self) -> bool {
        self.max_size == self.size
    }

    pub fn add(&mut self, value: (U256, IpAddr)) -> Result<(), BucketError> {
        if self.is_full() {
            return Err(BucketError::BucketFull);
        }

        self.value[self.size as usize] = value;
        self.size += 1;

        Ok(())
    }

    pub fn find_closest_node(&self, target: U256) -> [(U256, IpAddr); ALPHA] {
        let mut sorted = [(U256::default(), IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))); N_BUCKETS];

        sorted.copy_from_slice(&self.value);
        sorted.sort_by(|a, b| {
            let da = a.0 ^ target;
            let db = b.0 ^ target;

            da.cmp(&db)
        });

        let mut out = [(U256::default(), IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))); ALPHA];
        out.copy_from_slice(&sorted[..ALPHA]);

        out
    }
}
