//! Peer entry representation for Kademlia DHT nodes.
//!
//! Represents a single peer entry containing its identifier, IP address,
//! and computed XOR distance to a target node.
//!
//! # Peer Selection Strategy
//!
//! The module implements a hybrid peer selection strategy that balances two factors:
//! - **XOR Distance**: Primary metric for Kademlia routing (closer is better)
//! - **Response Time**: Secondary metric to avoid slow or overloaded peers
//!
//! The `distance_score` combines both metrics, ensuring that while distance remains
//! the dominant factor, peers with excessive response times are penalized. This prevents
//! selecting nodes that are topologically close but perform poorly, while still
//! prioritizing proximity in the Kademlia key space.

use crate::{S, T_MAX_MS};

use cryptography::U256;
use cryptography::hash::sha256;

use std::cmp::Ordering;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::time::Duration;

#[derive(Debug)]
pub enum EntryError {
    PingTimeout,
    Unreachable,
}

/// Represents a peer in the Kademlia network.
///
/// An entry contains:
/// - `id`: The peer's unique identifier (SHA256 hash of IP address)
/// - `addr`: The IP address (IPv4 or IPv6) of the peer
/// - `distance`: The XOR distance to a target node (computed on demand)
/// - `respond_time`: The round-trip time (latency) when pinging this peer
/// - `distance_score`: A combined metric incorporating both XOR distance and response time
#[derive(Clone, Copy)]
pub struct Entry {
    pub id: U256,
    pub addr: IpAddr,
    pub distance: U256,
    pub respond_time: Duration,
    pub distance_score: U256,
}

impl Entry {
    /// Creates a new entry from an IP address.
    ///
    /// Computes the entry's ID by hashing the IP address octets with SHA256.
    /// The distance is initialized to zero and computed later when needed.
    pub async fn new(addr: IpAddr) -> Result<Entry, EntryError> {
        let hash = match addr {
            IpAddr::V4(ip) => sha256(ip.octets().as_slice()),
            IpAddr::V6(ip) => sha256(ip.octets().as_slice()),
        };

        let mut entry = Entry {
            id: hash,
            addr,
            distance: U256::ZERO,
            respond_time: Duration::from_millis(0),
            distance_score: U256::ZERO,
        };

        entry.update_reponse_time().await?;

        Ok(entry)
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
    pub fn compare_distance(&self, target: &Entry) -> Ordering {
        self.distance.cmp(&target.distance)
    }

    /// Sends a ping request to the peer to verify its availability.
    ///
    /// # Returns
    /// * `Ok(())` if the peer responds successfully
    /// * `Err(EntryError)` if the peer is unreachable or times out
    pub async fn ping(&self) -> Result<(), EntryError> {
        Ok(())
    }

    /// Updates the response time metric for this peer.
    ///
    /// Measures the round-trip time by pinging the peer and stores the duration.
    /// This metric is used in calculating the peer's distance score.
    ///
    /// # Returns
    /// * `Ok(())` if the ping succeeds and the response time is updated
    /// * `Err(EntryError)` if the peer cannot be reached
    pub async fn update_reponse_time(&mut self) -> Result<(), EntryError> {
        self.ping().await?; // Add time wrapper
        // Update time response

        Ok(())
    }

    /// Calculates a time-based penalty for peer selection.
    ///
    /// Computes a penalty value based on the peer's response time and distance.
    /// Higher response times result in larger penalties, making slower peers
    /// less favorable in peer selection. The distance is right-shifted by S bits
    /// to weight the penalty appropriately.
    ///
    /// # Returns
    /// A U256 penalty value proportional to response time and distance
    fn time_penalty(&self) -> U256 {
        let d_part = self.distance >> S.into();

        let t_ms = self.respond_time.as_millis();
        let t_norm = t_ms.min(T_MAX_MS);

        d_part * U256::from(t_norm) / U256::from(T_MAX_MS)
    }

    /// Computes a combined distance score incorporating both XOR distance and response time.
    ///
    /// This score is used for peer selection in Kademlia operations. It combines:
    /// - The XOR distance to the target
    /// - A time penalty based on the peer's response latency
    ///
    /// Lower scores indicate more favorable peers (closer and faster).
    ///
    /// # Arguments
    /// * `target` - The target ID to measure distance to
    pub fn compute_distance_score(&mut self, target: U256) {
        self.compute_distance(target);
        self.distance_score = self.distance + self.time_penalty()
    }

    /// Compares the distance scores of two entries.
    ///
    /// Assumes `compute_distance_score` has been called beforehand on both entries.
    /// Used for sorting peers by their combined distance and latency metrics.
    ///
    /// # Arguments
    /// * `target` - The other entry to compare against
    ///
    /// # Returns
    /// An `Ordering` indicating whether this entry's score is less, equal, or greater
    pub fn compare_distance_score(&self, target: &Entry) -> Ordering {
        self.distance_score.cmp(&target.distance_score)
    }
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            id: U256::ZERO,
            addr: IpAddr::V4(Ipv4Addr::from_octets([0u8; 4])),
            distance: U256::ZERO,
            respond_time: Duration::from_millis(0),
            distance_score: U256::ZERO,
        }
    }
}
