use crate::U256;

use core::mem::{MaybeUninit, transmute};
use core::ops::RangeInclusive;

static K: u8 = 20;
static N_BUCKETS: usize = 256;

#[derive(Debug)]
pub struct Bucket {
    pub range: RangeInclusive<U256>,
    pub max_size: u8,
    pub size: u8,
    pub value: Box<[U256]>,
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
            value: Box::new([U256::default(); K as usize]),
        }
    }
}

#[derive(Debug)]
pub struct KBucket {
    pub buckets: [Bucket; N_BUCKETS],
}

impl KBucket {
    pub fn init(x: U256) -> Self {
        let mut out: [MaybeUninit<Bucket>; N_BUCKETS] =
            unsafe { MaybeUninit::uninit().assume_init() };

        for i in 0..=255 {
            out[i as usize] = MaybeUninit::new(Bucket::init(x, i));
        }

        Self {
            buckets: unsafe {
                transmute::<[MaybeUninit<Bucket>; N_BUCKETS], [Bucket; N_BUCKETS]>(out)
            },
        }
    }
}
