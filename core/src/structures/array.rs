//! Utility functions for efficient array operations.
//!
//! Provides algorithms for merging sorted collections and maintaining
//! sorted arrays of selected elements.

use std::cmp::Ordering;

/// Merges two sorted sequences into the first array, keeping the N smallest.
///
/// This is a critical operation in the Kademlia lookup algorithm. It efficiently
/// merges results from multiple bucket queries while maintaining only the top N
/// candidates by distance metric.
///
/// Algorithm:
/// 1. Performs a two-pointer merge on both sorted sequences
/// 2. Takes elements in ascending order until the first array is full (N elements)
/// 3. Returns the count of elements merged
///
/// # Type Parameters
/// * `T` - Element type (must be Copy for efficient array manipulation)
/// * `N` - Maximum capacity of the first array
/// * `F` - Comparator function type
///
/// # Arguments
/// * `s1` - Target array (must be pre-filled with initial values of size s1_size)
/// * `s1_size` - Number of valid elements currently in s1
/// * `s2` - Source array slice to merge
/// * `s2_size` - Number of valid elements to consider from s2
/// * `compare` - Comparator function defining sort order
///
/// # Returns
/// The number of elements in s1 after merge (≤ N)
///
/// # Example
/// Merge two lists of distances, keeping only the 4 (ALPHA) closest:
/// ```ignore
/// let count = keep_lowest_array_by(&mut closest, 2, &new_candidates, 3, |a, b| a.distance.cmp(&b.distance));
/// // Returns 4 if both arrays had enough elements
/// ```
pub fn keep_lowest_array_by<T: Copy, const N: usize, F>(
    s1: &mut [T; N],
    s1_size: usize,
    s2: &[T],
    s2_size: usize,
    compare: F,
) -> usize
where
    F: Fn(&T, &T) -> Ordering,
{
    let a = *s1;

    let mut i1 = 0usize;
    let mut i2 = 0usize;
    let mut k = 0usize;

    while k < N && (i1 < s1_size || i2 < s2_size) {
        let take_s1 =
            i1 < s1_size && (i2 >= s2_size || compare(&a[i1], &s2[i2]) != Ordering::Greater);

        let v = if take_s1 {
            let v = a[i1];
            i1 += 1;
            v
        } else {
            let v = s2[i2];
            i2 += 1;
            v
        };

        s1[k] = v;
        k += 1;
    }

    k
}
