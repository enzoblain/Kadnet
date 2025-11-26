use crate::U256;
use crate::bucket::KBucket;

#[derive(Debug)]
pub struct Node {
    pub id: U256,
    pub bucket: KBucket,
}

impl Node {
    pub fn generate_random() -> Self {
        let id = U256::generate_random();

        Self {
            id,
            bucket: KBucket::init(id),
        }
    }

    pub fn distance_from(&self, dest: &Node) -> U256 {
        self.id ^ dest.id
    }
}
