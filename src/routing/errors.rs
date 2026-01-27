use crate::network::errors::NetworkError;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum RoutingError {
    SelfNode,
    BucketError(BucketError),
    NetworkError(NetworkError),
}

#[derive(Debug)]
pub(crate) enum BucketError {
    NodeNotFound,
}
