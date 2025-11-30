use crate::{KUSIZE, U256, sha256_bytes};

use core::array::IntoIter;
use core::net::IpAddr;
use core::ops::{Index, IndexMut};
use core::slice::{Iter, IterMut};

#[derive(Default, Clone, Copy)]
pub struct Entries(pub [Option<Entry>; KUSIZE]); // Oldest -> Newest

impl Entries {
    pub fn add_entry(&mut self, addr: IpAddr, place: usize) {
        self[place] = Some(Entry::from(addr));
    }
}

impl Index<usize> for Entries {
    type Output = Option<Entry>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Entries {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Entries {
    pub fn iter(&self) -> Iter<'_, Option<Entry>> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Option<Entry>> {
        self.0.iter_mut()
    }
}

impl<'a> IntoIterator for &'a Entries {
    type Item = &'a Option<Entry>;
    type IntoIter = Iter<'a, Option<Entry>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Entries {
    type Item = &'a mut Option<Entry>;
    type IntoIter = IterMut<'a, Option<Entry>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl IntoIterator for Entries {
    type Item = Option<Entry>;
    type IntoIter = IntoIter<Option<Entry>, KUSIZE>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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
