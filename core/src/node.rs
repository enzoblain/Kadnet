use crate::U256;
use crate::bucket::KBucket;

#[derive(Debug)]
pub struct Node {
    pub id: U256,
    pub bucket: KBucket,
}

impl Node {
    pub fn generate_random() -> Self {
        Self {
            id: U256::generate_random(),
            bucket: KBucket::init(),
        }
    }

    pub fn distance_from(&self, dest: &Node) -> U256 {
        self.id ^ dest.id
    }
}
