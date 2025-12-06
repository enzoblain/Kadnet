//! Node components for Kademlia DHT network.
//!
//! Provides the core structures for a Kademlia node:
//! - `Node`: The main node structure managing all buckets and peer information
//! - `Bucket`: Contains entries for peers organized by XOR distance ranges
//! - `Entry`: Individual peer information with IP address and computed XOR distance
//!
//! The node organizes peers into buckets to efficiently manage and retrieve
//! the closest peers to any target identifier.

pub mod bucket;
pub mod core;
pub mod entry;
