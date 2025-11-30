use crate::{K, U256, sha256_bytes};

use core::net::IpAddr;

#[derive(Default, Clone, Copy)]
pub struct Entries(pub [Option<Entry>; K as usize]); // Oldest -> Newest

impl Entries {
    pub fn add_entry(&mut self, addr: IpAddr, place: usize) {
        self.0[place] = Some(Entry::from(addr));
    }
}
#[derive(Clone, Copy)]
pub struct Entry {
    pub id: U256,
    pub addr: IpAddr,
    pub distance: U256,
}

impl From<IpAddr> for Entry {
    fn from(value: IpAddr) -> Self {
        let hash = match value {
            IpAddr::V4(ip) => sha256_bytes(ip.octets().as_slice()),
            IpAddr::V6(ip) => sha256_bytes(ip.octets().as_slice()),
        };

        Entry {
            id: hash,
            addr: value,
            distance: U256::default(),
        }
    }
}

impl Entry {
    pub fn compute_distance(&mut self, target: U256) {
        self.distance = self.id ^ target;
    }
}
