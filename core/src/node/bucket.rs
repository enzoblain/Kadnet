use crate::node::entry::{Entry, EntryError};
use crate::structures::vecdeque::{SizedVecDeque, SizedVecDequeError};
use crate::{ALPHA, KUSIZE};

use cryptography::U256;

use std::net::IpAddr;

pub enum BucketError {
    SizedVecDequeError(SizedVecDequeError),
    OutOfRange,
    EntryError(EntryError),
}

pub struct Bucket {
    value: SizedVecDeque<Entry>,
}

impl Bucket {
    pub fn init(size: usize) -> Self {
        Self {
            value: SizedVecDeque::new(size.min(KUSIZE)),
        }
    }

    pub async fn add_entry(&mut self, addr: IpAddr) -> Result<(), BucketError> {
        let entry = Entry::new(addr).await.map_err(BucketError::EntryError)?;

        match self.value.insert(entry) {
            Ok(()) => Ok(()),
            Err(e) => Err(BucketError::SizedVecDequeError(e)),
        }
    }

    pub fn find_n_closest(&mut self, target: U256) -> ([Entry; ALPHA], usize) {
        let compute_distance = |e: &mut Entry| e.compute_distance_score(target);
        let compare_distance_score = |a: &Entry, b: &Entry| a.compare_distance_score(b);

        match self
            .value
            .compute_and_select_n_first_by::<ALPHA>(compute_distance, compare_distance_score)
        {
            Ok(res) => res,
            Err(_) => ([Entry::default(); ALPHA], 0),
        }
    }
}
