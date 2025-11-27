use crate::{K, U256, sha256_bytes};

use core::cmp::Ordering;
use core::net::IpAddr;

#[derive(Default)]
pub struct Entries([Option<Entry>; K as usize]); // Oldest -> Newest

impl Entries {
    pub fn sort_by_distance(&self, target: U256) -> Entries {
        let mut sorted = Entries::default();

        sorted.0.copy_from_slice(&self.0);
        sorted.0.sort_by(|a, b| match (a, b) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Greater,
            (Some(_), None) => Ordering::Less,
            (Some(a), Some(b)) => {
                let da = a.id ^ target;
                let db = b.id ^ target;

                da.cmp(&db)
            }
        });

        sorted
    }

    pub fn add_entry(&mut self, addr: IpAddr, place: usize) {
        self.0[place] = Some(Entry::from(addr));
    }
}

#[derive(Clone, Copy)]
struct Entry {
    id: U256,
    #[allow(dead_code)] // TODO: remove
    addr: IpAddr,
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
        }
    }
}
