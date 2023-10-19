pub fn binary_search<T: Ord>(arr: &[T], x: T) -> Option<usize> {
    if arr.is_empty() { return None; }
    
    let mut l = 0;
    let mut r = arr.len() - 1;
    
    while l < r {
        let mid = l+r >> 1;
        if arr[mid] == x { return Some(mid); }
        else if arr[mid] < x { l = mid + 1; }
        else { r = mid - 1; }
    }

    None
}
