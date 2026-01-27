pub(crate) mod network;
pub mod node;
pub(crate) mod routing;

pub(crate) mod consts {
    pub(crate) static ALPHA: usize = 4;
    pub(crate) static KUSIZE: usize = 32;
    pub(crate) static SMALL_BUCKET_COUNT: usize = 4;

    pub(crate) static N_BUCKETS: usize = 256;
    pub(crate) static DISTANCE_WEIGHT_SHIFT: u32 = 10;
    pub(crate) static PING_MAX_RETRY: usize = 3;
    pub(crate) static T_MAX_MS: u64 = 800;
}
