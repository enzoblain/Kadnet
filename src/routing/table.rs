use super::entry::NodeEntry;
use super::kbucket::{InsertDecision, KBucket};
use crate::consts::{ALPHA, KUSIZE, N_BUCKETS, SMALL_BUCKET_COUNT};
use crate::network;
use crate::routing::errors::RoutingError;

use cryptal::primitives::U256;
use std::array;

pub(crate) struct RoutingTable {
    local_id: U256,
    buckets: [KBucket; N_BUCKETS],
}

impl RoutingTable {
    pub(crate) fn new_from_id(id: U256) -> Self {
        let buckets: [KBucket; N_BUCKETS] = array::from_fn(|i| {
            let size = if i <= SMALL_BUCKET_COUNT {
                1usize << i
            } else {
                KUSIZE
            };

            KBucket::new(size)
        });

        Self {
            local_id: id,
            buckets,
        }
    }

    pub(crate) async fn insert(&mut self, entry: NodeEntry) -> Result<(), RoutingError> {
        let bucket_id = match self.find_corresponding_bucket(entry.id) {
            Some(bi) => bi,
            None => return Err(RoutingError::SelfNode),
        };

        match self.buckets[bucket_id].try_insert(entry) {
            InsertDecision::PingOldest(oldest) => {
                let ping = network::ping(oldest.addr)
                    .await
                    .map_err(RoutingError::NetworkError);

                if ping.is_err() {
                    let _ = self.buckets[bucket_id]
                        .remove(oldest)
                        .map_err(RoutingError::BucketError);
                    self.buckets[bucket_id].force_insert(entry);
                }

                Ok(())
            }

            InsertDecision::Inserted | InsertDecision::Refreshed => Ok(()),
        }
    }

    pub(crate) fn find_corresponding_bucket(&self, target: U256) -> Option<usize> {
        let distance = self.local_id ^ target;

        if distance == U256::ZERO {
            return None;
        }

        Some(N_BUCKETS - 1 - distance.leading_zeros() as usize)
    }

    pub(crate) fn get_closests(&mut self, target: U256) -> Vec<NodeEntry> {
        let bucket_number = self.find_corresponding_bucket(target).unwrap_or(0);

        let mut closests = self.buckets[bucket_number].select_n_closests(ALPHA, target);

        if closests.len() == ALPHA {
            return closests;
        }

        for d in 1..N_BUCKETS {
            let left = bucket_number as isize - d as isize;
            let right = bucket_number + d;

            if left >= 0 {
                let mut other_closests =
                    self.buckets[left as usize].select_n_closests(ALPHA, target);
                closests.append(&mut other_closests);
            }

            if closests.len() >= ALPHA {
                break;
            }

            if right < N_BUCKETS {
                let mut other_closests = self.buckets[right].select_n_closests(ALPHA, target);
                closests.append(&mut other_closests);
            }

            if closests.len() >= ALPHA {
                break;
            }
        }

        closests.truncate(ALPHA);

        closests
    }
}
