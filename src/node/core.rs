use super::errors::NodeError;
use crate::network::tcp;
use crate::routing::RoutingTable;
use crate::routing::entry::NodeEntry;
use crate::routing::id::generate_id;

use cadentis::select;
use cryptal::keys::ed25519;
use cryptal::primitives::U256;
use std::net::SocketAddr;
use std::sync::Mutex;
use std::sync::mpsc::{Receiver, channel};

pub struct Node {
    pub(crate) listenning_port: u16,
    pub(crate) routing: Mutex<RoutingTable>,
}

impl Node {
    pub(crate) fn new(listenning_port: u16, version: usize) -> Result<Self, NodeError> {
        let (pk, _sk) = ed25519::generate_keypair();
        let public_key = pk.to_bytes();

        let id = generate_id(public_key, version).map_err(NodeError::IdError)?;

        Ok(Self {
            listenning_port,
            routing: Mutex::new(RoutingTable::new_from_id(id)),
        })
    }

    pub async fn start(&mut self) -> Result<(), NodeError> {
        let (transmitter1, receiver1) = channel();
        let (transmitter2, receiver2) = channel();

        let listen = tcp::listen(self.listenning_port, transmitter1, transmitter2);
        let add_node = self.add_node(receiver1);
        let search = self.search_node(receiver2);

        select! {
            add_node => |_| println!("ah"),
            search => |_| println!("eh"),
            listen => |_| println!("oh"),
        }

        Ok(())
    }

    #[allow(clippy::await_holding_lock)]
    pub(crate) async fn add_node(&self, receiver: Receiver<(U256, SocketAddr)>) {
        while let Ok((id, addr)) = receiver.recv() {
            let entry = NodeEntry::new(id, addr).await.unwrap();

            let mut routing = self.routing.lock().unwrap();

            routing.insert(entry).await.unwrap();
        }
    }

    pub(crate) async fn search_node(&self, receiver: Receiver<U256>) {
        while let Ok(id) = receiver.recv() {
            let mut routing = self.routing.lock().unwrap();

            routing.get_closests(id);
        }
    }

    pub fn join() -> Result<(), NodeError> {
        Ok(())
    }

    pub fn stop() -> Result<(), NodeError> {
        Ok(())
    }
}
