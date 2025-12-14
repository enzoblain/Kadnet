//! The main Kademlia DHT node implementation.
//!
//! Provides the Node struct that manages multiple buckets and implements
//! Kademlia operations for finding the closest peers to a target ID.

use crate::node::bucket::Bucket;
use crate::node::entry::Entry;
use crate::structures::array::keep_lowest_array_by;
use crate::{ALPHA, KUSIZE, N_BUCKETS, SMALL_BUCKET_COUNT};

use core::array;
use cryptography::U256;
use cryptography::hash::sha256;

/// Represents a Kademlia DHT node.
///
/// Manages a network of peers organized into buckets based on XOR distance.
/// The node has 4 explicit small buckets (K=1,2,4,8) for close peers,
/// and 252 larger buckets (K=16) for more distant peers.
pub struct Node {
    pub id: U256,
    buckets: [Bucket; N_BUCKETS],
}

impl Node {
    /// Creates a new Kademlia node with the given seed value.
    ///
    /// Hashes the provided seed using SHA256 to generate the node's unique ID.
    /// Initializes N_BUCKETS (256) buckets with exponentially increasing capacities:
    /// - Buckets 0-4: sizes 1, 2, 4, 8, 16 (K value 1 to 16)
    /// - Buckets 5+: all have size KUSIZE (32) for efficient large-distance routing
    ///
    /// # Arguments
    /// * `val` - Seed bytes used to generate the node's unique identifier
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

    /// Finds the ALPHA closest peers to a target ID using a 2-way bucket search.
    ///
    /// Algorithm:
    /// 1. Determines the corresponding bucket for the target ID
    /// 2. Queries the corresponding bucket for closest entries
    /// 3. If result count < ALPHA, expands search to neighboring buckets (left/right)
    /// 4. Continues expanding until ALPHA entries are found or all buckets exhausted
    /// 5. Returns the top ALPHA entries sorted by XOR distance
    ///
    /// # Returns
    /// A tuple containing:
    /// - An array of up to ALPHA closest entries
    /// - The actual count of entries found (may be < ALPHA if network is sparse)
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

    /// Finds the bucket index corresponding to a target ID.
    ///
    /// Uses the XOR distance to determine which bucket a target ID belongs to.
    /// The bucket index is computed from the position of the most significant bit
    /// in the XOR distance.
    ///
    /// # Returns
    /// The index of the bucket containing entries closest to the target ID.
    fn find_corresponding_bucket(&self, target: U256) -> u8 {
        let distance = self.id ^ target;

        (N_BUCKETS - 1 - distance.leading_zeros() as usize) as u8
    }
}
