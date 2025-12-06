#![cfg_attr(feature = "no-std", no_std)]

//! Core Kademlia network node implementation with optional `no-std` support.
//!
//! Implements a Kademlia Distributed Hash Table (DHT) node that manages a network
//! of peers organized into buckets based on XOR distance metrics.
//! When the `no_std` feature is enabled, the node works without the standard library,
//! making it suitable for embedded and kernel environments.

/// Replication factor for Kademlia operations.
/// Used in node lookups and storage operations to ensure redundancy.
pub static ALPHA: usize = 4;

/// Size of larger buckets (K value as usize for array indexing).
static KUSIZE: usize = 16;

/// Maximum number of closest peers that can be collected from all buckets.
/// Used in no-std mode for stack-allocated buffers.
#[cfg(feature = "no-std")]
static MAX_CLOSEST: usize = 256 * ALPHA;

/// Number of active buckets with size K=16.
/// Excludes the first 4 special buckets (K=1,2,4,8) that have smaller capacities.
static N_BUCKETS_32: usize = 252;

pub mod node;

pub use node::core::Node;
