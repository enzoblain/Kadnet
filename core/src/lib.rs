//! Core Kademlia network node implementation.
//!
//! Implements a Kademlia Distributed Hash Table (DHT) node that manages a network
//! of peers organized into buckets based on XOR distance metrics.

/// Replication factor for Kademlia operations.
/// Used in node lookups and storage operations to ensure redundancy.
pub static ALPHA: usize = 4;

/// Size of larger buckets (K value as usize for array indexing).
static KUSIZE: usize = 32;

static SMALL_BUCKET_COUNT: usize = 4;

/// Number of active buckets with size K=16.
/// Excludes the first 4 special buckets (K=1,2,4,8) that have smaller capacities.
static N_BUCKETS: usize = 256;

pub mod node;
pub mod structures;

pub use node::core::Node;
