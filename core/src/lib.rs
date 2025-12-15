//! Core Kademlia network node implementation.
//!
//! Implements a Kademlia Distributed Hash Table (DHT) node that manages a network
//! of peers organized into buckets based on XOR distance metrics.

/// Replication factor for Kademlia operations.
/// Used in node lookups and storage operations to ensure redundancy.
pub static ALPHA: usize = 4;

/// Size of larger buckets (K value as usize for array indexing).
static KUSIZE: usize = 32;

/// Number of special buckets with reduced capacities.
/// The first 4 buckets have progressive sizes (K=1,2,4,8) instead of K=32.
static SMALL_BUCKET_COUNT: usize = 4;

/// Number of buckets
static N_BUCKETS: usize = 256;

/// Bit shift parameter for distance weighting in peer scoring.
/// Used to scale down distance values when calculating the time penalty component.
static S: u32 = 10;

/// Maximum timeout in milliseconds for network operations.
/// Maximum duration to wait for a response before considering a request failed.
static T_MAX_MS: u128 = 800;

pub mod node;
pub mod structures;

pub use node::core::Node;
