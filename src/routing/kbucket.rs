use crate::routing::errors::BucketErrors;

use super::entry::NodeEntry;

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

    pub(crate) fn remove(&mut self, entry: NodeEntry) -> Result<(), BucketErrors> {
        if let Some(position) = self.entries.iter().position(|ne| ne.id == entry.id) {
            self.entries.remove(position).unwrap();

            Ok(())
        } else {
            Err(BucketErrors::NodeNotFound)
        }
    }

    pub(crate) fn force_insert(&mut self, entry: NodeEntry) {
        self.entries.push_back(entry);
    }
}
