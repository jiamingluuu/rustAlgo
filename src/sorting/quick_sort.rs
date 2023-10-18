use std::mem::swap;

fn quick_sort<T: Ord + Copy>(arr: &mut [T], l: usize, r: usize) {
    if l >= r { return; }

    let p = arr[l+r >> 1];
    let mut i = l;
    let mut j = r;

    while i < j {
        while arr[i] < p { i += 1; }
        while arr[j] > p { j -= 1; }
        if i < j { 
            let tmp = arr[i];
            arr[i] = arr[j];
            arr[j] = tmp;
        }
    }

    quick_sort(arr, l, j);
    quick_sort(arr, j+1, r);
}

pub fn sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.is_empty() { return; }
    
    let n = arr.len();
    quick_sort(arr, 0, n-1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::helpers::{is_sorted, have_same_elements};

    #[test]
    fn basic() {
        let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        let cloned = res.clone();
        sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        let cloned = res.clone();
        sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        let cloned = res.clone();
        sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn one_element() {
        let mut res = vec![1];
        let cloned = res.clone();
        sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec![1, 2, 3, 4];
        let cloned = res.clone();
        sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn reverse_sorted() {
        let mut res = vec![4, 3, 2, 1];
        let cloned = res.clone();
        sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }
}
