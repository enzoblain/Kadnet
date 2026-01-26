use std::net::IpAddr;
use std::time::Duration;

use cryptal::primitives::U256;

use crate::consts::{DISTANCE_WEIGHT_SHIFT, T_MAX_MS};

#[derive(Clone, Copy)]
pub(crate) struct NodeEntry {
    pub(crate) id: U256,
    pub(crate) addr: IpAddr,

    pub(crate) score: U256,
    pub(crate) respond_time: Duration,
    pub(crate) distance: U256,
}

impl NodeEntry {
    pub(crate) fn distance(&self, target: U256) -> U256 {
        self.id ^ target
    }

    fn time_penalty(&self) -> U256 {
        let d_part = self.distance >> DISTANCE_WEIGHT_SHIFT.into();

        let t_ms = self.respond_time.as_millis();
        let t_norm = t_ms.min(T_MAX_MS as u128);

        d_part * U256::from(t_norm) / U256::from(T_MAX_MS)
    }

    pub(crate) fn compute_score(&mut self, target: U256) {
        self.score = self.distance(target) + self.time_penalty();
    }
}
