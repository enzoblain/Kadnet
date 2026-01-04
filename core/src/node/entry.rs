use crate::{S, T_MAX_MS};

use cryptal::hash::sha256;
use cryptal::primitives::U256;

use std::cmp::Ordering;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::time::Duration;

#[derive(Debug)]
pub enum EntryError {
    PingTimeout,
    Unreachable,
}

#[derive(Clone, Copy)]
pub struct Entry {
    pub id: U256,
    pub addr: IpAddr,
    pub distance: U256,
    pub respond_time: Duration,
    pub distance_score: U256,
}

impl Entry {
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

    pub fn compute_distance(&mut self, target: U256) {
        self.distance = self.id ^ target;
    }

    pub fn get_distance(&self) -> U256 {
        self.distance
    }

    pub fn compare_distance(&self, target: &Entry) -> Ordering {
        self.distance.cmp(&target.distance)
    }

    pub async fn ping(&self) -> Result<(), EntryError> {
        Ok(())
    }

    pub async fn update_reponse_time(&mut self) -> Result<(), EntryError> {
        self.ping().await?; // Add time wrapper
        // Update time response

        Ok(())
    }

    fn time_penalty(&self) -> U256 {
        let d_part = self.distance >> S.into();

        let t_ms = self.respond_time.as_millis();
        let t_norm = t_ms.min(T_MAX_MS);

        d_part * U256::from(t_norm) / U256::from(T_MAX_MS)
    }

    pub fn compute_distance_score(&mut self, target: U256) {
        self.compute_distance(target);
        self.distance_score = self.distance + self.time_penalty()
    }

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
