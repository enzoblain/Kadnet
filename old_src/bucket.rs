use crate::node::entry::Entry;
use crate::structures::vecdeque::SizedVecDeque;
use crate::{ALPHA, KUSIZE};

use cryptal::primitives::U256;

use std::net::IpAddr;

pub struct Bucket {
    value: SizedVecDeque<Entry>,
}

impl Bucket {
    pub fn init(size: usize) -> Self {
        Self {
            value: SizedVecDeque::new(size.min(KUSIZE)),
        }
    }

    pub async fn add_entry(&mut self, addr: IpAddr) -> Result<(), ()> {
        let entry = Entry::new(addr).await?;
        self.value.insert(entry)?;

        Ok(())
    }

    pub fn find_n_closest(&mut self, target: U256) -> Vec<Entry> {
        let compute_distance = |e: &mut Entry| e.compute_distance_score(target);
        let compare_distance_score = |a: &Entry, b: &Entry| a.compare_distance_score(b);

        self.value
            .select_n_first_by(ALPHA, compute_distance, compare_distance_score)
    }
}
