pub(crate) enum RoutingErrors {
    SelfNode,
    BucketError(BucketErrors),
}

pub(crate) enum BucketErrors {
    NodeNotFound,
}
