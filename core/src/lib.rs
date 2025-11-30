#![no_std]

pub static K: u8 = 20;
pub static KUSIZE: usize = 20;
pub static N_BUCKETS: usize = 256;
pub static ALPHA: usize = 3;

pub mod math;
pub mod node;

use math::hash::sha256_bytes;
use math::u256::U256;
