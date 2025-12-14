//! Kademlia bucket for organizing and managing peer entries.
//!
//! A bucket stores peer entries (nodes) within a specific XOR distance range.
//! Each bucket has a fixed maximum capacity K and maintains entries sorted by distance.

use crate::node::entry::Entry;
use crate::structures::vecdeque::{SizedVecDeque, SizedVecDequeError};
use crate::{ALPHA, KUSIZE};

use cryptography::U256;

use std::net::IpAddr;

/// Errors that can occur during bucket operations.
pub enum BucketError {
    /// Error from the underlying sized vector deque structure
    SizedVecDequeError(SizedVecDequeError),
    /// Entry is outside the acceptable range for this bucket
    OutOfRange,
}

/// A bucket for storing peer entries in the Kademlia DHT.
///
/// Each bucket covers a specific range of node IDs and stores up to K entries.
/// The range is determined by the node's own ID and the bucket's index.
pub struct Bucket {
    value: SizedVecDeque<Entry>,
}

impl Bucket {
    /// Initializes a new bucket with a specified capacity.
    ///
    /// Creates a bucket that can store up to `size` entries (capped at KUSIZE).
    /// The bucket maintains entries in a sized vector deque for efficient insertion
    /// and distance-based ordering.
    ///
    /// # Arguments
    /// * `size` - Maximum number of entries this bucket can hold (clamped to KUSIZE)
    pub fn init(size: usize) -> Self {
        Self {
            value: SizedVecDeque::new(size.min(KUSIZE)),
        }
    }

    /// Adds a new peer entry with the given IP address to the bucket.
    ///
    /// Returns `Ok(())` on success or `LinkedListError` if the bucket is full.
    pub fn add_entry(&mut self, addr: IpAddr) -> Result<(), BucketError> {
        let entry = Entry::new(addr);

        match self.value.insert(entry) {
            Ok(()) => Ok(()),
            Err(e) => Err(BucketError::SizedVecDequeError(e)),
        }
    }

    /// Finds up to ALPHA closest entries to a target ID within this bucket.
    ///
    /// Algorithm:
    /// 1. Computes XOR distance from each entry to the target
    /// 2. Selects the ALPHA entries with smallest distances
    /// 3. Returns them sorted by ascending distance
    ///
    /// # Arguments
    /// * `target` - The target ID to measure distance against
    ///
    /// # Returns
    /// A tuple containing:
    /// - Array of up to ALPHA closest entries
    /// - Actual count of entries returned (≤ ALPHA)
    pub fn find_n_closest(&mut self, target: U256) -> ([Entry; ALPHA], usize) {
        let compute_distance = |e: &mut Entry| e.compute_distance(target);
        let compare_distance = |a: &Entry, b: &Entry| a.distance.cmp(&b.distance);

        match self
            .value
            .compute_and_select_n_first_by::<ALPHA>(compute_distance, compare_distance)
        {
            Ok(res) => res,
            Err(_) => ([Entry::default(); ALPHA], 0),
        }
    }
}
