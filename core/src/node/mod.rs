use crate::{N_BUCKETS, U256};
use bucket::Bucket;
use entry::Entries;

use core::array;

pub mod bucket;
pub mod entry;

pub struct Node {
    pub id: U256,
    pub bucket: [Bucket; N_BUCKETS],
}

impl Node {
    pub fn generate_random() -> Self {
        let id = U256::generate_random();

        Self {
            id,
            bucket: array::from_fn(|i| Bucket::init(id, i as u8)),
        }
    }

    pub fn distance_from(&self, dest: &Node) -> U256 {
        self.id ^ dest.id
    }
}
