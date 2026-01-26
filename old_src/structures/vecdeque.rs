use std::cmp::Ordering;
use std::collections::VecDeque;

pub(crate) struct SizedVecDeque<T> {
    inner: VecDeque<T>,
}

impl<T> SizedVecDeque<T> {
    pub(crate) fn new(size: usize) -> Self {
        SizedVecDeque {
            inner: VecDeque::with_capacity(size),
        }
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn is_full(&self) -> bool {
        self.len() == self.inner.capacity()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub(crate) fn insert(&mut self, value: T) -> Result<(), ()> {
        if self.is_full() {
            return Err(());
        }

        self.inner.push_front(value);

        Ok(())
    }

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
