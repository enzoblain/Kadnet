use crate::node::bucket::Bucket;
use crate::node::entry::Entry;
use crate::{ALPHA, KUSIZE, N_BUCKETS, SMALL_BUCKET_COUNT};

use core::array;
use cryptal::hash::sha256;
use cryptal::primitives::U256;

pub struct Node {
    pub id: U256,
    buckets: [Bucket; N_BUCKETS],
}

impl Node {
    pub(crate) fn new(val: &[u8]) -> Self {
        let id = sha256(val);
        let buckets: [Bucket; N_BUCKETS] = array::from_fn(|i| {
            let size = if i <= SMALL_BUCKET_COUNT {
                1usize << i
            } else {
                KUSIZE
            };

            Bucket::init(size)
        });

        Self { id, buckets }
    }

    pub(crate) fn get_closests(&mut self, target: U256) -> Vec<Entry> {
        let bucket_number = self.find_corresponding_bucket(target) as isize;

        let mut closests = self.buckets[bucket_number as usize].find_n_closest(target);

        if closests.len() == ALPHA {
            return closests;
        }

        for d in 1..N_BUCKETS {
            let left = bucket_number - d as isize;
            let right = bucket_number + d as isize;

            if left >= 0 {
                let mut other_closests = self.buckets[left as usize].find_n_closest(target);
                closests.append(&mut other_closests);
            }

            if closests.len() >= ALPHA {
                break;
            }

            if right < N_BUCKETS as isize {
                let mut other_closests = self.buckets[left as usize].find_n_closest(target);
                closests.append(&mut other_closests);
            }

            if closests.len() >= ALPHA {
                break;
            }
        }

        closests.truncate(ALPHA);

        closests
    }

    fn find_corresponding_bucket(&self, target: U256) -> u8 {
        let distance = self.id ^ target;

        (N_BUCKETS - 1 - distance.leading_zeros() as usize) as u8
    }
}
