//! Fixed-capacity vector deque with distance-based selection.
//!
//! Provides a bounded vector deque that supports efficient insertion, removal,
//! and selection of N smallest elements based on a comparator function.

use std::cmp::Ordering;
use std::collections::VecDeque;

/// Errors that can occur during sized deque operations.
pub enum SizedVecDequeError {
    /// The deque has reached maximum capacity
    IsFull,
    /// Index is out of bounds for the current deque
    IndexOutOfRange,
    /// Cannot perform operation on empty deque
    IsEmpty,
}

/// A VecDeque with fixed maximum capacity.
///
/// Maintains entries with O(1) insertion at the front and provides efficient
/// selection of N smallest elements based on distance comparisons.
pub struct SizedVecDeque<T> {
    inner: VecDeque<T>,
}

impl<T> SizedVecDeque<T> {
    /// Creates a new sized deque with the specified capacity.
    ///
    /// # Arguments
    /// * `size` - Maximum number of elements this deque can hold
    pub fn new(size: usize) -> Self {
        SizedVecDeque {
            inner: VecDeque::with_capacity(size),
        }
    }

    /// Returns the current number of elements in the deque.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Checks if the deque has reached its maximum capacity.
    pub fn is_full(&self) -> bool {
        self.len() == self.inner.capacity()
    }

    /// Checks if the deque contains no elements.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Validates that an index is within bounds.
    fn check_bounds(&self, index: usize) -> Result<(), SizedVecDequeError> {
        if index >= self.len() {
            return Err(SizedVecDequeError::IndexOutOfRange);
        }

        Ok(())
    }

    /// Inserts an element at the front of the deque if not full.
    ///
    /// # Returns
    /// - `Ok(())` on successful insertion
    /// - `Err(IsFull)` if the deque is at maximum capacity
    pub fn insert(&mut self, value: T) -> Result<(), SizedVecDequeError> {
        if self.is_full() {
            return Err(SizedVecDequeError::IsFull);
        }

        self.inner.push_front(value);

        Ok(())
    }

    /// Removes and returns the element at the specified index.
    ///
    /// # Arguments
    /// * `index` - Position of element to remove
    ///
    /// # Returns
    /// - `Ok(T)` with the removed element
    /// - `Err(IndexOutOfRange)` if index is out of bounds
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

    /// Computes a value for each element and returns the N smallest by comparator.
    ///
    /// This is the core operation for finding closest peers in Kademlia.
    /// Each element is transformed via `compute()`, then the N elements with
    /// smallest values according to `compare()` are selected and sorted.
    ///
    /// # Type Parameters
    /// * `N` - Number of elements to select
    ///
    /// # Arguments
    /// * `compute` - Function to compute/update each element before comparison
    /// * `compare` - Comparator function (returns Ordering::Less for smaller values)
    ///
    /// # Returns
    /// - `Ok((array, count))` - Array of selected elements and actual count
    /// - `Err(IsEmpty)` if the deque is empty
    ///
    /// # Generic Constraint
    /// * `T` must be `Copy` to allow multiple comparisons and movements
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
