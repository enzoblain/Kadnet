use std::net::SocketAddr;

use cryptal::primitives::U256;

#[derive(PartialEq, Eq)]
pub(crate) enum Rpc {
    Ping,
    Pong,
    Connect((U256, SocketAddr)),
    Search(U256),
}

impl Rpc {
    pub(crate) fn as_byte(&self) -> u8 {
        match self {
            Rpc::Ping => 0,
            Rpc::Pong => 1,
            Rpc::Connect(_) => 2,
            Rpc::Search(_) => 3,
        }
    }

    pub(crate) fn from_byte(byte: &[u8]) -> Option<Self> {
        match byte[0] {
            0 => Some(Rpc::Ping),
            1 => Some(Rpc::Pong),
            2 => Some(Rpc::Connect((U256::ZERO, "127.0.0.1:0".parse().unwrap()))),
            3 => Some(Rpc::Search(U256::ZERO)),
            _ => None,
        }

        // The others bytes could contain the content
    }
}
