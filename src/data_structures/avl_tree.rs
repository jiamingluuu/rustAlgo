use std::cmp::{Ordering, max};
use std::mem;

struct AVL_Node<T: Ord> {
    value: T,
    height: usize,
    left: Option<Box<AVL_Node<T>>>,
    right: Option<Box<AVL_Node<T>>>,
}

pub struct AVL_Tree<T: Ord> {
    root: Option<Box<AVL_Node<T>>>,
    length: usize,
}

impl<T: Ord> AVL_Tree<T> {
    pub fn new() -> AVL_Tree<T> {
        AVL_Tree { root: None, length: 0 }
    }
    
    pub fn contains(&self, value: T) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            current = match value.cmp(&node.value) {
                Ordering::Equal => return false,
                Ordering::Greater => &node.right,
                Ordering::Less => &node.left,
            }
        }

        false
    }
    
    pub fn insert(&mut self, value: T) -> bool {
        if self.is_empty() {
            *self = AVL_Tree {
                root: Some(AVL_Node::new(value)),
                length: 1,
            };
            return true;
        } 
        
        if AVL_Node::insert(&mut self.root, value) {
            self.length += 1;
            return true;
        } 

        false
    }
    
    pub fn remove(&self, value: T) {

    }
    
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

impl<T: Ord> AVL_Node<T> {
    pub fn new(value: T) -> Box<AVL_Node<T>> {
        Box::new(AVL_Node { value, height: 1, left: None, right: None })
    }
    
    pub fn insert(root: &mut Option<Box<AVL_Node<T>>>, value: T) -> bool {
        if root.is_none() {
            *root = Some(Self::new(value));
            return true;
        }

        let mut node = root.unwrap();
        let inserted = match node.value.cmp(&value) {
            Ordering::Equal => return false,
            Ordering::Greater => Self::insert(&mut node.left, value),
            Ordering::Less => Self::insert(&mut node.right, value),
        };
        
        if inserted { node.rebalance(); }

        inserted
    }
    
    pub fn remove(&mut self, value: T) -> bool {
        todo!()
    }

    fn rebalance(&mut self) {
        let balance_factor = self.balance_factor();
        if balance_factor > 1 {
            let left = *self.left.unwrap();

            if (Self::height(left.left) >= Self::height(left.right)) {
                self.left_rotation();
            } else {
                self.left_right_rotation();
            }

        } else if balance_factor < 1 {
            let right = *self.right.unwrap();
            
            if (Self::height(right.right) >= Self::height(right.left)) {
                self.right_rotation();
            } else {
                self.right_left_rotation();
            }
        }
    }
    
    fn balance_factor(&self) -> i8 {
        let left = match &self.left {
            Some(node) => node.height,
            None => 0,
        };
        let right = match &self.right {
            Some(node) => node.height,
            None => 0,
        };
        
        if left < right {
            return -((right - left) as i8);
        } else {
            return (left - right) as i8;
        }
    }
    
    /*
     *                T                           R
     *              /   \                       /   \
     *             L     R        --->         T     RR  
     *                  / \                   / \
     *                 RL  RR                L   RL
     *          
     */
    pub fn left_rotation(&mut self) {
        let mut right = self.right.take().unwrap();
        self.right = right.left.take();
        self.update_height();

        mem::swap(self, right.as_mut());

        self.left = Some(right);
        self.update_height();
    }
    
    pub fn right_rotation(&mut self) {
        let mut left = self.left.take().unwrap();
        self.left = left.right.take();
        self.update_height();

        mem::swap(self, left.as_mut());
        
        self.right = Some(left);
        self.update_height();
    }
    
    pub fn left_right_rotation(&mut self) {
        let mut left = self.left.take().unwrap();
        left.left_rotation();
        self.left = Some(left);
        self.right_rotation();
    }
    
    pub fn right_left_rotation(&mut self) {
        let mut right = self.right.take().unwrap();
        right.right_rotation();
        self.right = right;
        self.left_rotation();
    }

    pub fn update_height(&mut self) {
        let left_height = match &self.left {
            Some(node) => node.height,
            None => 0,
        };

        let right_height = match &self.right {
            Some(node) => node.height,
            None => 0,
        };

        self.height = 1 + max(left_height, right_height);
    }
    
    fn height(node: Option<Box<AVL_Node<T>>>) -> usize {
        match node {
            Some(node) => node.height,
            None => 0,
        }
    }
}