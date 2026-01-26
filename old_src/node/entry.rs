use crate::{S, T_MAX_MS};

use cryptal::hash::sha256;
use cryptal::primitives::U256;

use std::cmp::Ordering;
use std::net::IpAddr;
use std::time::Duration;

#[derive(Clone, Copy)]
pub(crate) struct Entry {
    id: U256,
    addr: IpAddr,
    distance: U256,
    respond_time: Duration,
    distance_score: U256,
}

impl Entry {
    pub async fn new(addr: IpAddr) -> Result<Entry, ()> {
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

    pub(crate) fn compute_distance(&mut self, target: U256) {
        self.distance = self.id ^ target;
    }

    pub(crate) async fn ping(&self) -> Result<(), ()> {
        Ok(())
    }

    pub(crate) async fn update_reponse_time(&mut self) -> Result<(), ()> {
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

    pub(crate) fn compute_distance_score(&mut self, target: U256) {
        self.compute_distance(target);
        self.distance_score = self.distance + self.time_penalty()
    }

    pub fn compare_distance_score(&self, target: &Entry) -> Ordering {
        self.distance_score.cmp(&target.distance_score)
    }
}
