#![allow(dead_code)]
pub struct Heap<T> {
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T> {
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            items: vec![],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // returns the element at the top of the heap
    pub fn peek(&self) -> Option<&T> {
        self.items.get(0)
    }

    pub fn push(&mut self, item: T) {
        let mut idx: usize = self.len();
        self.items.push(item);
        while let Some(parent_idx) = self.parent(idx) {
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx);
            }
            idx = parent_idx;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() { return None; }

        let top: Option<T> = Some(self.items.swap_remove(0));

        if self.is_empty() { return top; }

        let mut current_idx: usize = 0;
        let mut left_idx: usize  = self.left_child(current_idx);
        let mut right_idx: usize  = self.right_child(current_idx);
        let mut smaller_child: usize;

        while  self.is_valid_idx(left_idx) {
            smaller_child = {
                if (self.comparator)(&self.items[left_idx], 
                                     &self.items[current_idx]) {
                    left_idx
                } else if self.is_valid_idx(right_idx) 
                    && (self.comparator)(&self.items[right_idx], 
                                         &self.items[current_idx]) {
                    right_idx
                } else { 
                    // none of children is smaller, heap property updated
                    break;
                }
            };

            self.items.swap(current_idx, smaller_child);

            current_idx = smaller_child;
            left_idx = self.left_child(current_idx);
            right_idx = self.right_child(current_idx);
        }

        top
    }

    fn parent(&self, idx: usize) -> Option<usize> {
        // check idx > 0 to make sure item[idx] is not the root of the heap
        if idx > 0 {
            Some((idx-1) / 2)
        } else {
            None
        }
    }

    fn left_child(&self, idx: usize) -> usize {
        idx*2 + 1
    }

    fn right_child(&self, idx: usize) -> usize {
        idx*2 + 2
    }

    fn is_valid_idx(&self, idx: usize) -> bool {
        idx <= self.len()-1
    }
}

impl<T: Ord> Heap<T> {
    pub fn new_min() -> Heap<T> {
        Heap::new(|x, y| x < y)
    }

    pub fn new_max() -> Heap<T> {
        Heap::new(|x, y| x > y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap: Heap<i32> = Heap::new_max();
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = Heap::new_min();
        heap.push(4);
        heap.push(2);
        heap.push(9);
        heap.push(11);
        println!("\nheap: {:?}\n", heap.items);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.pop(), Some(2));
        println!("\nheap: {:?}\n", heap.items);
        assert_eq!(heap.pop(), Some(4));
        println!("\nheap: {:?}\n", heap.items);
        assert_eq!(heap.pop(), Some(9));
        println!("\nheap: {:?}\n", heap.items);
        heap.push(1);
        assert_eq!(heap.pop(), Some(1));
        println!("\nheap: {:?}\n", heap.items);
    }

    #[test]
    fn test_max_heap() {
        let mut heap = Heap::new_max();
        heap.push(4);
        heap.push(2);
        heap.push(9);
        heap.push(11);
        assert_eq!(heap.len(), 4);
        println!("\nheap: {:?}\n", heap.items);
        assert_eq!(heap.pop(), Some(11));
        println!("\nheap: {:?}\n", heap.items);
        assert_eq!(heap.pop(), Some(9));
        println!("\nheap: {:?}\n", heap.items);
        assert_eq!(heap.pop(), Some(4));
        println!("\nheap: {:?}\n", heap.items);
        heap.push(1);
        println!("\nheap: {:?}\n", heap.items);
        assert_eq!(heap.pop(), Some(2));
    }

    struct Point(/* x */ i32, /* y */ i32);

    #[test]
    fn test_key_heap() {
        let mut heap: Heap<Point> = Heap::new(|a, b| a.0 < b.0);
        heap.push(Point(1, 5));
        heap.push(Point(3, 10));
        heap.push(Point(-2, 4));
        assert_eq!(heap.len(), 3);
        assert_eq!(heap.pop().unwrap().0, -2);
        assert_eq!(heap.pop().unwrap().0, 1);
        heap.push(Point(50, 34));
        assert_eq!(heap.pop().unwrap().0, 3);
    }

    // #[test]
    // fn test_iter_heap() {
    //     let mut heap = Heap::new_min();
    //     heap.push(4);
    //     heap.push(2);
    //     heap.push(9);
    //     heap.push(11);

    //     // test iterator, which is not in order except the first one.
    //     let mut iter = heap.iter();
    //     assert_eq!(iter.next(), Some(&2));
    //     assert_ne!(iter.next(), None);
    //     assert_ne!(iter.next(), None);
    //     assert_ne!(iter.next(), None);
    //     assert_eq!(iter.next(), None);

    //     // test the heap after run iterator.
    //     assert_eq!(heap.len(), 4);
    //     assert_eq!(heap.pop(), Some(2));
    //     assert_eq!(heap.pop(), Some(4));
    //     assert_eq!(heap.pop(), Some(9));
    //     assert_eq!(heap.pop(), Some(11));
    //     assert_eq!(heap.pop(), None);
    // }
}
