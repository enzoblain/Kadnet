use crate::id::U256;

use core::mem::{MaybeUninit, transmute};

static K: u8 = 20;
static N_BUCKETS: usize = 256;

#[derive(Debug)]
pub struct Bucket {
    pub bottom_value: U256,
    pub top_value: U256,
    pub max_size: u8,
    pub size: u8,
    pub value: Box<[U256]>,
}

impl Bucket {
    pub fn init(k: u8) -> Self {
        let max_size = k.min(K);

        let bottom_value = U256::two_pow_k(k);
        let top_value = if k != 255 {
            U256::two_pow_k(k + 1) - U256::from(1)
        } else {
            U256::MAX
        };

        Self {
            bottom_value,
            top_value,
            max_size,
            size: 0,
            value: Box::new([U256::default(); K as usize]),
        }
    }
}

#[derive(Debug)]
pub struct KBucket(pub [Bucket; N_BUCKETS]);

impl KBucket {
    pub fn init() -> Self {
        let mut out: [MaybeUninit<Bucket>; N_BUCKETS] =
            unsafe { MaybeUninit::uninit().assume_init() };

        for i in 0..=255 {
            out[i as usize] = MaybeUninit::new(Bucket::init(i));
        }

        Self(unsafe { transmute::<[MaybeUninit<Bucket>; N_BUCKETS], [Bucket; N_BUCKETS]>(out) })
    }
}
