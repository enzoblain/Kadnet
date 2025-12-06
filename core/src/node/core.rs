//! The main Kademlia DHT node implementation.
//!
//! Provides the Node struct that manages multiple buckets and implements
//! Kademlia operations for finding the closest peers to a target ID.

use crate::node::bucket::Bucket;
use crate::node::entry::Entry;
use crate::{KUSIZE, N_BUCKETS_32};

#[cfg(feature = "no-std")]
use crate::MAX_CLOSEST;

use cryptography::U256;
use cryptography::hash::sha256;

use core::array;
use core::mem::MaybeUninit;

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
    pub fn get_n_closest<const N: usize>(&mut self, target: U256) -> [MaybeUninit<Entry>; N] {
        let mut entries = Vec::new();

        entries.extend(self.bucket1.find_n_closest::<N>(target).0);
        entries.extend(self.bucket2.find_n_closest::<N>(target).0);
        entries.extend(self.bucket3.find_n_closest::<N>(target).0);
        entries.extend(self.bucket4.find_n_closest::<N>(target).0);

        for bucket in self.buckets.iter_mut() {
            entries.extend(bucket.find_n_closest::<N>(target).0);
        }

        entries.sort_by(|a, b| a.distance.cmp(&b.distance));

        let mut out: [MaybeUninit<Entry>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        for (i, entry) in entries.into_iter().take(N).enumerate() {
            out[i] = MaybeUninit::new(entry);
        }

        out
    }

    /// Finds the N closest peers to a target ID (no-std version).
    ///
    /// Searches all buckets and returns an array of the N closest entries.
    /// Uses a fixed-size buffer for stack allocation without heap allocation.
    #[cfg(feature = "no-std")]
    pub fn get_n_closest<const N: usize>(&mut self, target: U256) -> [MaybeUninit<Entry>; N] {
        let mut temp: [MaybeUninit<Entry>; MAX_CLOSEST] =
            unsafe { MaybeUninit::uninit().assume_init() };
        let mut len = 0;

        macro_rules! append_bucket {
            ($bucket:expr) => {
                let (entries, n) = $bucket.find_n_closest::<N>(target);
                for i in 0..n {
                    temp[len] = entries[i];
                    len += 1;
                }
            };
        }

        append_bucket!(self.bucket1);
        append_bucket!(self.bucket2);
        append_bucket!(self.bucket3);
        append_bucket!(self.bucket4);

        for bucket in self.buckets.iter_mut() {
            append_bucket!(bucket);
        }

        for i in 1..len {
            let mut j = i;

            while j > 0 {
                unsafe {
                    let a = temp[j - 1].assume_init_ref();
                    let b = temp[j].assume_init_ref();

                    if a.distance <= b.distance {
                        break;
                    }

                    temp.swap(j - 1, j);
                }

                j -= 1;
            }
        }

        let mut out: [MaybeUninit<Entry>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            if i < len {
                out[i] = temp[i];
            } else {
                out[i] = MaybeUninit::uninit();
            }
        }

        out
    }
}
