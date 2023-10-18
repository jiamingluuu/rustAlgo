use std::cmp;

pub fn is_sorted<T: Ord>(arr: &[T]) -> bool {
    if arr.is_empty() { return true; }
    let len = arr.len();
    for i in 0..len - 1 {
        if arr[i] > arr[i+1] {
            return false;
        }
    }
    true
}

pub fn have_same_elements<T>(a: &[T], b: &[T]) -> bool
where
    // T: cmp::PartialOrd,
    // If HashSet is used
    T: cmp::PartialOrd + cmp::Eq + std::hash::Hash,
{
    // if one of a, b is empty but the another is not
    if a.is_empty() ^ b.is_empty() { return false; }

    use std::collections::HashSet;

    match a.len() == b.len() {
        true => {
            // This is O(n^2) but performs better on smaller data sizes
            //b.iter().all(|item| a.contains(item))

            // This is O(n), performs well on larger data sizes
            let set_a: HashSet<&T> = a.iter().collect();
            let set_b: HashSet<&T> = b.iter().collect();
            set_a == set_b
        }
        false => false,
    }
}
