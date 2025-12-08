//! The main Kademlia DHT node implementation.
//!
//! Provides the Node struct that manages multiple buckets and implements
//! Kademlia operations for finding the closest peers to a target ID.

use crate::node::bucket::Bucket;
use crate::node::entry::Entry;
use crate::{KUSIZE, N_BUCKETS_32};

use core::array;
use cryptography::U256;
use cryptography::hash::sha256;

#[cfg(feature = "no-std")]
use datastructures::array::core::keep_lowest_array_by as keep_lowest_by;

#[cfg(feature = "no-std")]
use datastructures::option::core::put_option_last;

#[cfg(not(feature = "no-std"))]
use datastructures::vec::core::keep_lowest_vec_by as keep_lowest_by;

/// Represents a Kademlia DHT node.
///
/// Manages a network of peers organized into buckets based on XOR distance.
/// The node has 4 explicit small buckets (K=1,2,4,8) for close peers,
/// and 252 larger buckets (K=16) for more distant peers.
pub struct Node {
    pub id: U256,

    bucket1: Bucket<1>,
    bucket2: Bucket<2>,
    bucket3: Bucket<4>,
    bucket4: Bucket<8>,

    buckets: [Bucket<KUSIZE>; N_BUCKETS_32],
}

impl Node {
    /// Creates a new Kademlia node with the given identifier.
    ///
    /// Initializes all buckets based on the node's SHA256 hashed ID.
    /// The ID uniquely identifies this node in the DHT network.
    pub fn new(val: &[u8]) -> Self {
        let id = sha256(val);

        let bucket1 = Bucket::<1>::init(id, 0);
        let bucket2 = Bucket::<2>::init(id, 1);
        let bucket3 = Bucket::<4>::init(id, 2);
        let bucket4 = Bucket::<8>::init(id, 3);

        // Start at bucket number 4 (after the first four explicit buckets) to keep the
        // computed bucket indices within the u8 range and avoid overflow when i is large.
        let buckets: [Bucket<KUSIZE>; N_BUCKETS_32] =
            array::from_fn(|i| Bucket::<KUSIZE>::init(id, i as u8 + 4));

        Self {
            id,
            bucket1,
            bucket2,
            bucket3,
            bucket4,
            buckets,
        }
    }

    /// Finds the N closest peers to a target ID (std version).
    ///
    /// Searches all buckets and returns an array of the N closest entries.
    /// Entries are sorted by XOR distance to the target.
    #[cfg(not(feature = "no-std"))]
    pub fn get_n_closest<const N: usize>(&mut self, target: U256) -> Vec<Entry> {
        let mut closest = self.bucket1.find_n_closest::<N>(target);

        keep_lowest_by(
            &mut closest,
            self.bucket2.find_n_closest::<N>(target),
            |a, b| a.compare_distance(b),
        );
        keep_lowest_by(
            &mut closest,
            self.bucket3.find_n_closest::<N>(target),
            |a, b| a.compare_distance(b),
        );
        keep_lowest_by(
            &mut closest,
            self.bucket4.find_n_closest::<N>(target),
            |a, b| a.compare_distance(b),
        );

        for bucket in self.buckets.iter_mut() {
            keep_lowest_by(&mut closest, bucket.find_n_closest::<N>(target), |a, b| {
                a.compare_distance(b)
            });
        }

        closest
    }

    /// Finds the N closest peers to a target ID (no-std version).
    ///
    /// Searches all buckets and returns an array of the N closest entries.
    /// Uses a fixed-size buffer for stack allocation without heap allocation.
    #[cfg(feature = "no-std")]
    pub fn get_n_closest<const N: usize>(&mut self, target: U256) -> [Option<Entry>; N] {
        let mut closest = self.bucket1.find_n_closest::<N>(target);

        keep_lowest_by(
            &mut closest,
            self.bucket2.find_n_closest::<N>(target),
            |a, b| put_option_last(a, b, |aa, bb| aa.compare_distance(bb)),
        );
        keep_lowest_by(
            &mut closest,
            self.bucket3.find_n_closest::<N>(target),
            |a, b| put_option_last(a, b, |aa, bb| aa.compare_distance(bb)),
        );
        keep_lowest_by(
            &mut closest,
            self.bucket4.find_n_closest::<N>(target),
            |a, b| put_option_last(a, b, |aa, bb| aa.compare_distance(bb)),
        );

        for bucket in self.buckets.iter_mut() {
            keep_lowest_by(&mut closest, bucket.find_n_closest::<N>(target), |a, b| {
                put_option_last(a, b, |aa, bb| aa.compare_distance(bb))
            });
        }

        closest
    }
}
