use std::cmp::Ordering;
use std::collections::VecDeque;

pub enum SizedVecDequeError {
    IsFull,
    IndexOutOfRange,
    IsEmpty,
}

pub struct SizedVecDeque<T> {
    inner: VecDeque<T>,
}

impl<T> SizedVecDeque<T> {
    pub fn new(size: usize) -> Self {
        SizedVecDeque {
            inner: VecDeque::with_capacity(size),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_full(&self) -> bool {
        self.len() == self.inner.capacity()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn check_bounds(&self, index: usize) -> Result<(), SizedVecDequeError> {
        if index >= self.len() {
            return Err(SizedVecDequeError::IndexOutOfRange);
        }

        Ok(())
    }

    pub fn insert(&mut self, value: T) -> Result<(), SizedVecDequeError> {
        if self.is_full() {
            return Err(SizedVecDequeError::IsFull);
        }

        self.inner.push_front(value);

        Ok(())
    }

    pub fn remove(&mut self, index: usize) -> Result<T, SizedVecDequeError> {
        self.check_bounds(index)?;

        Ok(self.inner.remove(index).unwrap())
    }

    pub fn get(&self, index: usize) -> Result<&T, SizedVecDequeError> {
        self.check_bounds(index)?;

        Ok(self.inner.get(index).unwrap())
    }

    pub fn get_mut(&mut self, index: usize) -> Result<&mut T, SizedVecDequeError> {
        self.check_bounds(index)?;

        Ok(self.inner.get_mut(index).unwrap())
    }

    pub fn iter_and_compute<F>(&mut self, f: F)
    where
        F: Fn(&mut T),
    {
        for value in self.inner.iter_mut() {
            f(value)
        }
    }

    pub fn compute_and_select_n_first_by<const N: usize>(
        &self,
        compute: impl Fn(&mut T),
        compare: impl Fn(&T, &T) -> Ordering,
    ) -> Result<([T; N], usize), SizedVecDequeError>
    where
        T: Copy,
    {
        let len = self.len();
        let size = len.min(N);

        if len == 0 {
            return Err(SizedVecDequeError::IsEmpty);
        }

        let mut out: [T; N] = [self.inner[0]; N];
        let mut filled = 0;

        for item in self.inner.iter() {
            let mut computed_item = *item;
            compute(&mut computed_item);

            let mut pos = filled;
            while pos > 0 && compare(&computed_item, &out[pos - 1]) == Ordering::Less {
                pos -= 1;
            }

            if pos < N {
                let end = filled.min(N - 1);
                for i in (pos + 1..=end).rev() {
                    out[i] = out[i - 1];
                }

                out[pos] = computed_item;
                filled = (filled + 1).min(N);
            }
        }

        Ok((out, size))
    }
}
