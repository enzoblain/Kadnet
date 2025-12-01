use super::bucket::Bucket;
use super::entry::{Entries, Entry};
use crate::{KUSIZE, N_BUCKETS, U256};

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

        let mut size = 0;
        let mut max_value: Option<Entry> = None;

        let last = KUSIZE - 1;

        for array in self.bucket.iter() {
            for mut item in array.value.into_iter().flatten() {
                item.compute_distance(target);

                if max_value.is_some_and(|mv| item.distance >= mv.distance) {
                    continue;
                }

                for i in 0..KUSIZE {
                    if let Some(ref_item) = out[i] {
                        if item.distance == ref_item.distance {
                            break;
                        }

                        if item.distance < ref_item.distance {
                            for k in (i + 1..KUSIZE).rev() {
                                out[k] = out[k - 1];
                            }

                            out[i] = Some(item);
                            break;
                        }
                    } else {
                        out[i] = Some(item);
                        size += 1;

                        if size == KUSIZE {
                            max_value = out[last];
                        }

                        break;
                    }
                }
            }
        }

        out
    }
}
