use super::bucket::Bucket;
use super::entry::{Entries, Entry};
use crate::{K, N_BUCKETS, U256};

use core::array;

pub struct Node {
    pub id: U256,
    pub bucket: [Bucket; N_BUCKETS],
}

impl Node {
    pub fn generate_random() -> Self {
        let id = U256::generate_random();

        Self {
            id,
            bucket: array::from_fn(|i| Bucket::init(id, i as u8)),
        }
    }

    pub fn get_k_closest(&self, target: U256) -> Entries {
        let mut out = Entries::default();

        let mut size: u8 = 0;
        let mut max_value: Option<Entry> = None;

        let last = K - 1;

        for array in self.bucket.iter() {
            for mut item in array.value.0.into_iter().flatten() {
                item.compute_distance(target);

                if let Some(ma) = max_value {
                    if item.distance >= ma.distance {
                        continue;
                    }
                }

                for i in 0..K {
                    if let Some(ref_item) = out.0[i as usize] {
                        if item.distance == ref_item.distance {
                            break;
                        }

                        if item.distance < ref_item.distance {
                            for k in (i + 1..K).rev() {
                                out.0[k as usize] = out.0[k as usize - 1];
                            }

                            out.0[i as usize] = Some(item);
                            break;
                        }
                    } else {
                        out.0[i as usize] = Some(item);
                        size += 1;

                        if size == K {
                            max_value = out.0[last as usize];
                        }

                        break;
                    }
                }
            }
        }

        out
    }
}
