use super::entry::NodeEntry;
use super::errors::BucketError;

use cryptal::primitives::U256;
use std::collections::VecDeque;

pub(crate) enum InsertDecision {
    Inserted,
    PingOldest(NodeEntry),
    Refreshed,
}

pub(crate) struct KBucket {
    entries: VecDeque<NodeEntry>,
    capacity: usize,
}

impl KBucket {
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            entries: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    fn is_full(&self) -> bool {
        self.entries.len() == self.capacity
    }

    pub(crate) fn try_insert(&mut self, entry: NodeEntry) -> InsertDecision {
        if let Some(position) = self.entries.iter().position(|ne| ne.id == entry.id) {
            let ne = self.entries.remove(position).unwrap();
            self.entries.push_back(ne);

            return InsertDecision::Refreshed;
        }

        if !self.is_full() {
            self.entries.push_back(entry);
            InsertDecision::Inserted
        } else {
            InsertDecision::PingOldest(*self.entries.front().unwrap())
        }
    }

    pub(crate) fn remove(&mut self, entry: NodeEntry) -> Result<(), BucketError> {
        if let Some(position) = self.entries.iter().position(|ne| ne.id == entry.id) {
            self.entries.remove(position).unwrap();

            Ok(())
        } else {
            Err(BucketError::NodeNotFound)
        }
    }

    pub(crate) fn force_insert(&mut self, entry: NodeEntry) {
        self.entries.push_back(entry);
    }

    pub(crate) fn select_n_closests(&self, n: usize, target: U256) -> Vec<NodeEntry> {
        let mut out = Vec::with_capacity(n);

        for item in self.entries.iter() {
            let mut computed = *item;
            computed.compute_score(target);

            let pos = out
                .iter()
                .position(|ne: &NodeEntry| computed.score < ne.score)
                .unwrap_or(out.len());

            if pos < n {
                if out.len() == n {
                    out.pop();
                }

                out.insert(pos, *item);
            }
        }

        out
    }
}
