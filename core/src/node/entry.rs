//! Peer entry representation for Kademlia DHT nodes.
//!
//! Represents a single peer entry containing its identifier, IP address,
//! and computed XOR distance to a target node.

use cryptography::U256;
use cryptography::hash::sha256;

use core::cmp::Ordering;
use core::net::IpAddr;
use std::net::Ipv4Addr;

/// Represents a peer in the Kademlia network.
///
/// An entry contains:
/// - `id`: The peer's unique identifier (SHA256 hash of IP address)
/// - `addr`: The IP address (IPv4 or IPv6) of the peer
/// - `distance`: The XOR distance to a target node (computed on demand)
#[derive(Clone, Copy)]
pub struct Entry {
    pub id: U256,
    pub addr: IpAddr,
    pub distance: U256,
}

impl Entry {
    /// Creates a new entry from an IP address.
    ///
    /// Computes the entry's ID by hashing the IP address octets with SHA256.
    /// The distance is initialized to zero and computed later when needed.
    pub fn new(addr: IpAddr) -> Entry {
        let hash = match addr {
            IpAddr::V4(ip) => sha256(ip.octets().as_slice()),
            IpAddr::V6(ip) => sha256(ip.octets().as_slice()),
        };

        Entry {
            id: hash,
            addr,
            distance: U256::default(),
        }
    }

    /// Computes and caches the XOR distance to a target ID.
    ///
    /// The Kademlia metric uses XOR distance for determining node proximity.
    /// This method updates the entry's cached distance field, which is then used
    /// for sorting and selecting closest peers.
    ///
    /// # Arguments
    /// * `target` - The target ID to measure distance to
    pub fn compute_distance(&mut self, target: U256) {
        self.distance = self.id ^ target;
    }

    /// Returns the XOR distance to the target node.
    pub fn get_distance(&self) -> U256 {
        self.distance
    }

    /// Compares the computed distance of two entries.
    /// Assumes `compute_distance` has been called beforehand on both entries.
    pub fn compare_distance(&self, target: &Self) -> Ordering {
        self.distance.cmp(&target.distance)
    }
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            id: U256::ZERO,
            addr: IpAddr::V4(Ipv4Addr::from_octets([0u8; 4])),
            distance: U256::ZERO,
        }
    }
}
