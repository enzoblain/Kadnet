use crate::consts::{PING_MAX_RETRY, T_MAX_MS};
use crate::network::errors::NetworkError;
use crate::network::rpc::Rpc;
use crate::network::tcp::send_rpc;

use cadentis::task;
use cadentis::time::{instrumented, timeout};
use cadentis::tools::retry;
use std::net::SocketAddr;
use std::time::Duration;

pub(crate) async fn ping(addr: SocketAddr) -> Result<Duration, NetworkError> {
    let (res, duration) = instrumented(
        retry(PING_MAX_RETRY, move || {
            task::spawn(async move {
                let rpc = timeout(Duration::from_millis(T_MAX_MS), send_rpc(addr, Rpc::Ping))
                    .await
                    .map_err(|_| NetworkError::Timeout)??;

                match rpc {
                    Rpc::Pong => Ok(()),
                    _ => Err(NetworkError::WrongRPC),
                }
            })
        })
        .set_interval(Duration::from_millis(200)),
    )
    .await;

    res.map(|_| duration)
}
