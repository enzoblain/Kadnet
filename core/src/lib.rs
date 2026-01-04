pub static ALPHA: usize = 4;
static KUSIZE: usize = 32;
static SMALL_BUCKET_COUNT: usize = 4;
static N_BUCKETS: usize = 256;
static S: u32 = 10;
static T_MAX_MS: u128 = 800;

pub mod node;
pub mod structures;

pub use node::core::Node;
