use crate::node::bucket::Bucket;
use crate::node::entry::Entry;
use crate::structures::array::keep_lowest_array_by;
use crate::{ALPHA, KUSIZE, N_BUCKETS, SMALL_BUCKET_COUNT};

use core::array;
use cryptography::U256;
use cryptography::hash::sha256;

pub struct Node {
    pub id: U256,
    buckets: [Bucket; N_BUCKETS],
}

impl Node {
    pub fn new(val: &[u8]) -> Self {
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

    pub fn get_closests(&mut self, target: U256) -> ([Entry; ALPHA], usize) {
        let bucket_number = self.find_corresponding_bucket(target) as isize;

        let (mut closests, size) = self.buckets[bucket_number as usize].find_n_closest(target);

        if size == ALPHA {
            return (closests, ALPHA);
        }

        let mut other_closests = [Entry::default(); ALPHA];
        let mut other_size = 0;

        let compare_distance = |a: &Entry, b: &Entry| a.distance.cmp(&b.distance);

        for d in 1..N_BUCKETS {
            let left = bucket_number - d as isize;
            let right = bucket_number + d as isize;

            if left >= 0 {
                let (c, s) = self.buckets[left as usize].find_n_closest(target);
                other_size =
                    keep_lowest_array_by(&mut other_closests, other_size, &c, s, compare_distance);
            }

            if size + other_size >= ALPHA {
                break;
            }

            if right < N_BUCKETS as isize {
                let (c, s) = self.buckets[right as usize].find_n_closest(target);
                other_size =
                    keep_lowest_array_by(&mut other_closests, other_size, &c, s, compare_distance);
            }

            if size + other_size >= ALPHA {
                break;
            }
        }

        let final_size = keep_lowest_array_by(
            &mut closests,
            size,
            &other_closests,
            other_size,
            compare_distance,
        );

        (closests, final_size)
    }

    fn find_corresponding_bucket(&self, target: U256) -> u8 {
        let distance = self.id ^ target;

        (N_BUCKETS - 1 - distance.leading_zeros() as usize) as u8
    }
}
