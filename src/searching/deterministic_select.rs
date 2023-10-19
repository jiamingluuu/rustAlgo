/* The running time of deterministic_select is O(n), this is because:
 *  - The length of `less` and `greater` sub-array is at most floor(3n/4).
 *  - Hence, let T(n) be the running time of the algorithm, and we have:
 *
 *              {   c,                                      if n < 50
 *      T(n) <= |
 *              {   T(floor(n/5)) + T(floor(3n/4)) + cn,    otherwise 
 *
 *    Therefore, by complete induction on n, we can prove that:
 *
 *    T(n)  <= T(floor(n/5)) + T(floor(3n/4)) + cn
 *          <= 20c * floor(n/5) + 20c * floor(3n/4) + cn
 *          <= 20c * n/5 + 20c * 3n/4 + cn
 *          = 4cn + 15cn + cn
 *          = 20cn
 */
pub fn deterministic_select<T: Ord + Copy>(arr: &Vec<T>, k: usize) -> T {
    // if arr.len() < 50 {
    //     let mut arr_copy = arr;
    //     arr_copy.sort();
    //     return arr[k-1];
    // }

    let mut medians: Vec<T> = Vec::new();
    let mut sub_arr: Vec<T> = Vec::with_capacity(5);
    let mut j: usize = 0;
    for item in arr {
        sub_arr[j] = item.clone();
        j += 1;
        if j == 0 {
            j %= 5;
            medians.push(median(sub_arr.clone()));
        }
    }
    
    let pivot = deterministic_select(&medians, medians.len() / 2);
    let mut less: Vec<T> = Vec::new();
    let mut greater: Vec<T> = Vec::new();
    for item in arr {
        if item > &pivot { greater.push(item.clone()); }
        else { less.push(item.clone()); }
    }

    if k < less.len() { return deterministic_select(&less, k); }
    else if k == less.len() { return less[k-1]; }
    else { return deterministic_select(&greater, k - less.len() - 1); }
}

fn median<T: Ord + Copy>(mut arr: Vec<T>) -> T {
    arr.sort();
    arr[2]
}

#[cfg(test)] 
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        let mut sorted = res.clone();
        sorted.sort();
        assert_eq!(deterministic_select(&res, 5), sorted[5]);
        assert_eq!(deterministic_select(&res, 0), sorted[0]);
        assert_eq!(deterministic_select(&res, 10), sorted[10]);
    }
}
