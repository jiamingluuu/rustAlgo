fn merge_sort<T: Ord + Copy>(arr: &mut [T], l: usize, r: usize) -> () {
    if l >= r { return; }

    let mid: usize = (l + r) / 2;

    merge_sort(arr, l, mid);
    merge_sort(arr, mid+1, r);

    let mut tmp = arr.to_vec();
    let mut i: usize = l;
    let mut j: usize = mid+1;
    let mut k: usize = 0;

    while i <= mid && j <= r {
        if arr[i] > arr[j] { 
            tmp[k] = arr[j]; 
            j += 1; 
        } else { 
            tmp[k] = arr[i]; 
            i += 1; 
        }

        k += 1;
    }

    while i <= mid { 
        tmp[k] = arr[i]; 
        i += 1; 
        k += 1;
    }

    while j <= r {
        tmp[k] = arr[j];
        k += 1;
        j += 1; 
    }

    let mut i = 0;
    for j in l..=r {
        arr[j] = tmp[i];
        i += 1;
    }
}

pub fn sort<T: Ord + Copy>(arr: &mut [T]) {
     if arr.is_empty() {
         return;
     }

    let n = arr.len();
    merge_sort(arr, 0, n-1);
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
