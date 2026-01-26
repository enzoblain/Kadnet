use super::entry::NodeEntry;
use super::kbucket::{InsertDecision, KBucket};
use crate::consts::{KUSIZE, N_BUCKETS, SMALL_BUCKET_COUNT, T_MAX_MS};
use crate::routing::errors::RoutingErrors;

use cadentis::time::timeout;
use cadentis::tools::retry;
use cryptal::primitives::U256;
use std::array;
use std::time::Duration;

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

    pub(crate) async fn insert(&mut self, entry: NodeEntry) -> Result<(), RoutingErrors> {
        let bucket_id = match self.find_corresponding_bucket(entry.id) {
            Some(bi) => bi,
            None => return Err(RoutingErrors::SelfNode),
        };

        match self.buckets[bucket_id].try_insert(entry) {
            InsertDecision::PingOldest(oldest) => {
                let ping = retry(3, async move || {
                    timeout(Duration::from_micros(T_MAX_MS), async {
                        {
                            // ping the node: Network.ping(entry.id)
                        }
                    })
                    .await
                })
                .set_interval(Duration::from_micros(200))
                .await;

                if ping.is_err() {
                    let _ = self.buckets[bucket_id]
                        .remove(oldest)
                        .map_err(RoutingErrors::BucketError);
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
}
