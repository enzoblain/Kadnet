use super::errors::NodeError;
use crate::routing::RoutingTable;
use crate::routing::id::generate_id;

use cryptal::keys::ed25519;
use cryptal::primitives::U256;

pub struct Node {
    pub(crate) id: U256,
    pub(crate) routing: RoutingTable,

    pub(crate) version: usize,
}

impl Node {
    pub(crate) fn new(version: usize) -> Result<Self, NodeError> {
        let (pk, _sk) = ed25519::generate_keypair();
        let public_key = pk.to_bytes();

        let id = generate_id(public_key, version).map_err(NodeError::IdError)?;

        Ok(Self {
            id,
            routing: RoutingTable::new_from_id(id),
            version,
        })
    }

    pub fn start() -> Result<(), NodeError> {
        Ok(())
    }

    pub fn join() -> Result<(), NodeError> {
        Ok(())
    }

    pub fn stop() -> Result<(), NodeError> {
        Ok(())
    }
}
