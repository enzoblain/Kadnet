use std::cmp::Ordering;

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
