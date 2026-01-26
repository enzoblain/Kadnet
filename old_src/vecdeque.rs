impl<T> SizedVecDeque<T> {
    pub(crate) fn select_n_first_by(
        &self,
        n: usize,
        mut compute: impl FnMut(&mut T),
        compare: impl Fn(&T, &T) -> Ordering,
    ) -> Vec<T>
    where
        T: Clone,
    {
        let mut out = Vec::with_capacity(n);

        for item in self.inner.iter() {
            let mut computed = item.clone();
            compute(&mut computed);

            let pos = out
                .iter()
                .position(|x| compare(&computed, x) == Ordering::Less)
                .unwrap_or(out.len());

            if pos < n {
                if out.len() == n {
                    out.pop();
                }

                out.insert(pos, computed);
            }
        }

        out
    }
}
