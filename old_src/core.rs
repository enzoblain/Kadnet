impl Node {
    pub(crate) fn get_closests(&mut self, target: U256) -> Vec<Entry> {
        let bucket_number = self.find_corresponding_bucket(target) as isize;

        let mut closests = self.buckets[bucket_number as usize].find_n_closest(target);

        if closests.len() == ALPHA {
            return closests;
        }

        for d in 1..N_BUCKETS {
            let left = bucket_number - d as isize;
            let right = bucket_number + d as isize;

            if left >= 0 {
                let mut other_closests = self.buckets[left as usize].find_n_closest(target);
                closests.append(&mut other_closests);
            }

            if closests.len() >= ALPHA {
                break;
            }

            if right < N_BUCKETS as isize {
                let mut other_closests = self.buckets[left as usize].find_n_closest(target);
                closests.append(&mut other_closests);
            }

            if closests.len() >= ALPHA {
                break;
            }
        }

        closests.truncate(ALPHA);

        closests
    }
}
