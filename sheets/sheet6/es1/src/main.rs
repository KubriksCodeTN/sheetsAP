use std::{cmp::Ordering, fmt::Display};

#[derive(Debug)]
struct TreeNode<T: PartialOrd + Clone> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: PartialOrd + Clone> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode { value, left: None, right: None }
    }

    fn insert(&mut self, value: T) {
        match self.value.partial_cmp(&value) {
            Some(Ordering::Greater) => {
                if self.left.is_none() {
                    self.left = Some(Box::new(TreeNode::new(value)));
                    return;
                }
                TreeNode::insert(self.left.as_mut().unwrap(), value);
            }
            _ => {
                if self.right.is_none() {
                    self.right = Some(Box::new(TreeNode::new(value)));
                    return;
                }
                TreeNode::insert(self.right.as_mut().unwrap(), value);
            }
        }
    }

    fn from_vec(v: Vec<T>) -> Self {
        if v.len() == 0 {
            panic!();
        }
        let mut it: std::vec::IntoIter<T> = v.into_iter();
        let mut ret: TreeNode<T> = TreeNode::new(it.next().unwrap());

        for e in it {
            ret.insert(e);
        }

        ret
    }
}

impl<T: PartialOrd + Clone + Display> TreeNode<T> {
    fn preorder(&self) {
        println!("{}", self.value);
        if self.left.is_some() {
            TreeNode::preorder(self.left.as_ref().unwrap());
        }
        if self.right.is_some() {
            TreeNode::preorder(self.right.as_ref().unwrap());
        }
    }

    fn postorder(&self) {
        if self.right.is_some() {
            TreeNode::postorder(self.right.as_ref().unwrap());
        }
        println!("{}", self.value);
        if self.left.is_some() {
            TreeNode::postorder(self.left.as_ref().unwrap());
        }
    }

    fn inorder(&self) {
        if self.left.is_some() {
            TreeNode::inorder(self.left.as_ref().unwrap());
        }
        println!("{}", self.value);
        if self.right.is_some() {
            TreeNode::inorder(self.right.as_ref().unwrap());
        }
    }
}

fn main() {
    TreeNode::preorder(&TreeNode::from_vec(vec![1, 4, 3, 6, 9, 0]));
    println!();
    TreeNode::inorder(&TreeNode::from_vec(vec![1, 4, 3, 6, 9, 0]));
    println!();
    TreeNode::postorder(&TreeNode::from_vec(vec![1, 4, 3, 6, 9, 0]));
}
