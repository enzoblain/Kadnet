use cadentis::net::{TcpListener, TcpStream};
use cadentis::task;
use cadentis::time::timeout;
use cryptal::primitives::U256;
use std::net::SocketAddr;
use std::sync::mpsc::Sender;
use std::time::Duration;

use crate::network::errors::NetworkError;
use crate::network::rpc::Rpc;

pub(crate) async fn listen(
    port: u16,
    transmitter1: Sender<(U256, SocketAddr)>,
    transmitter2: Sender<U256>,
) -> Result<(), NetworkError> {
    let listener =
        TcpListener::bind(&format!("127.0.0.1:{port}")).map_err(|_| NetworkError::Connection)?;

    loop {
        let (stream, addr) = listener
            .accept()
            .await
            .map_err(|_| NetworkError::Connection)?;

        let tx1 = transmitter1.clone();
        let tx2 = transmitter2.clone();
        task::spawn(async move {
            if let Some(rpc) = parse_stream(stream, addr).await {
                match rpc {
                    Rpc::Ping => {
                        let _ = send_rpc(addr, Rpc::Pong).await?;
                    }
                    Rpc::Connect(node) => tx1.send(node).map_err(|_| NetworkError::Send)?,
                    Rpc::Search(node) => tx2.send(node).map_err(|_| NetworkError::Send)?,
                    _ => {}
                }
            }

            Ok::<(), NetworkError>(())
        })
        .await?;
    }
}

async fn parse_stream(stream: TcpStream, addr: SocketAddr) -> Option<Rpc> {
    let mut first = [0u8; 1];
    match stream.read(&mut first).await {
        Ok(0) => return None,
        Ok(_) => {}
        Err(_) => return None,
    }

    let rpc = Rpc::from_byte(&first[..0])?;
    if ![Rpc::Ping, Rpc::Pong].contains(&rpc) {
        return Some(rpc);
    }

    let max_len;
    let max_time;

    match rpc {
        Rpc::Connect(_) => {
            max_len = 32;
            max_time = Duration::from_millis(400);
        }
        _ => return None,
    }

    let result = timeout(max_time, async {
        let mut buf = vec![0u8; max_len];
        let mut read_bytes = 0;

        while read_bytes < max_len {
            match stream.read(&mut buf[read_bytes..]).await {
                Ok(0) => return None,
                Ok(n) => read_bytes += n,
                Err(_) => return None,
            }
        }

        let number: [u8; 32] = buf[..32].try_into().unwrap();

        Some(Rpc::Connect((U256::from(number), addr)))
    })
    .await;

    result.unwrap_or_default()
}

pub(crate) async fn send_rpc(addr: SocketAddr, rpc: Rpc) -> Result<Rpc, NetworkError> {
    let addr_str = addr.to_string();

    let stream = TcpStream::connect(&addr_str)
        .await
        .map_err(|_| NetworkError::Connection)?;

    stream
        .write_all(&[rpc.as_byte()])
        .await
        .map_err(|_| NetworkError::Write)?;

    let mut buf = [0u8; 1024];
    let n = stream
        .read(&mut buf)
        .await
        .map_err(|_| NetworkError::Read)?;

    let response = Rpc::from_byte(&buf[..n]);

    match response {
        Some(r) => Ok(r),
        None => Err(NetworkError::WrongRPC),
    }
}
