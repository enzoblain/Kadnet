#![no_std]

pub static K: u8 = 20;
pub static N_BUCKETS: usize = 256;

pub mod collections;
pub mod node;

use collections::u256::U256;
