//! Kademlia bucket for organizing and managing peer entries.
//!
//! A bucket stores peer entries (nodes) within a specific XOR distance range.
//! Each bucket has a fixed maximum capacity K and maintains entries sorted by distance.

use super::entry::Entry;

use cryptography::U256;
use datastructures::Const;
use datastructures::DoubleLinkedList::SizedDoubleLinkedList;
use datastructures::DoubleLinkedList::sized::ValidK;
use datastructures::LinkedListError;

use core::net::IpAddr;
use core::ops::Range;

pub enum BucketError {
    LinkedListError(LinkedListError),
    OutOfRange,
}

/// A bucket for storing peer entries in the Kademlia DHT.
///
/// Each bucket covers a specific range of node IDs and stores up to K entries.
/// The range is determined by the node's own ID and the bucket's index.
pub struct Bucket<const K: usize>
where
    Const<K>: ValidK,
{
    range: Range<U256>,
    value: SizedDoubleLinkedList<Entry, K>,
}

impl<const K: usize> Bucket<K>
where
    Const<K>: ValidK,
{
    /// Initializes a new bucket for a given node ID and bucket number.
    ///
    /// Computes the XOR distance range that this bucket covers based on the node's ID
    /// and the bucket number. The range determines which peers belong to this bucket.
    pub fn init(id: U256, bucket_number: u8) -> Self {
        let bucket_number = U256::from(bucket_number);
        let one = U256::from(1u32);

        let bottom_value = ((id >> (bucket_number + one)) << (bucket_number + one))
            + ((one - ((id >> bucket_number) & one)) << bucket_number);
        let top_value = bottom_value + (one << bucket_number);

        Self {
            range: bottom_value..top_value,
            value: SizedDoubleLinkedList::<Entry, K>::default(),
        }
    }

    /// Adds a new peer entry with the given IP address to the bucket.
    ///
    /// Returns `Ok(())` on success or `LinkedListError` if the bucket is full.
    pub fn add_entry(&mut self, addr: IpAddr) -> Result<(), BucketError> {
        let entry = Entry::new(addr);

        if !self.range.contains(&entry.id) {
            return Err(BucketError::OutOfRange);
        }

        match self.value.insert_tail(entry) {
            Ok(_) => Ok(()),
            Err(e) => Err(BucketError::LinkedListError(e)),
        }
    }

    /// Computes the XOR distance from each entry in the bucket to a target ID.
    pub fn compute_distance(&mut self, target: U256) {
        self.value.iter_and_compute(|e| e.compute_distance(target));
    }

    /// Finds the N closest entries to a target ID (no-std version).
    ///
    /// Returns a tuple of (uninitialized array of entries, count of found entries).
    /// The returned array may contain more entries than the actual count;
    /// only the first `count` entries are valid.
    #[cfg(feature = "no-std")]
    pub fn find_n_closest<const N: usize>(&mut self, target: U256) -> [Option<Entry>; N] {
        self.compute_distance(target);

        self.value
            .select_n_first_by::<N>(|a, b| a.distance.cmp(&b.distance))
    }

    /// Finds the N closest entries to a target ID (std version).
    ///
    /// Returns a tuple of (vector of entries, count of found entries).
    #[cfg(not(feature = "no-std"))]
    pub fn find_n_closest<const N: usize>(&mut self, target: U256) -> Vec<Entry> {
        self.compute_distance(target);

        self.value
            .select_n_first_by::<N>(|a, b| a.distance.cmp(&b.distance))
    }
}
