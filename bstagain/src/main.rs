#![allow(dead_code)]

use std::cmp::Ordering;

type NodeRef<T> = Box<Node<T>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    left: Option<NodeRef<T>>,
    right: Option<NodeRef<T>>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Self { elem, left: None, right: None }
    }
}

// maybe PartialOrd? but prob Ord is better
#[derive(Debug)]
struct Bst<T: Ord> {
    root: Option<NodeRef<T>>,
}

impl<T: Ord> Default for Bst<T> {
    fn default() -> Self {
        Self { root: None }
    }
}

impl<T: Ord> Bst<T> {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, elem: T) {
        let mut tmp = &mut self.root;

        while let Some(n) = tmp {
            match elem.cmp(&n.elem) {
                Ordering::Less => tmp = &mut n.left,
                _ => tmp = &mut n.right,
            }
        }

        *tmp = Some(Box::new(Node::new(elem)));
    }

    fn search(&self, elem: T) -> bool {
        let mut tmp = &self.root;

        while let Some(n) = tmp {
            match elem.cmp(&n.elem) {
                Ordering::Less => tmp = &n.left,
                Ordering::Greater => tmp = &n.right,
                Ordering::Equal => return true,
            }
        }

        false
    }

    fn get_min_above(mut root: &mut Option<Box<Node<T>>>) -> T {
        while root.as_ref().unwrap().left.is_some() {
            root = &mut root.as_mut().unwrap().left;
        }

        let tmp = root.take().unwrap();
        *root = tmp.right;
        tmp.elem
    }

    fn delete(root: &mut Option<Box<Node<T>>>, e: &T) -> bool {
        let n: &mut Box<Node<T>>;

        match root {
            None => return false,
            Some(node) => n = node,
        }

        match e.cmp(&n.elem) {
            Ordering::Equal => {
                match (&n.left, &n.right) {
                    (Some(_), Some(_)) => n.elem = Self::get_min_above(&mut n.right),
                    (Some(_), None) => *root = n.left.take(),
                    (_, _) => *root = n.right.take(),
                }
                return true;
            }
            Ordering::Less => Self::delete(&mut n.left, e),
            Ordering::Greater => Self::delete(&mut n.right, e),
        }
    }

    fn remove(&mut self, elem: &T) -> bool {
        Self::delete(&mut self.root, elem)
    }
}

fn main() {
    let mut bst: Bst<&str> = Bst::new();
    bst.insert("5");
    bst.insert("3");
    bst.insert("6");
    bst.insert("4");
    bst.insert("7");
    println!("{:?}", bst);
    assert_eq!(bst.search("7"), true);
    assert_eq!(bst.search("3"), true);
    assert_eq!(bst.search("33"), false);
    assert_eq!(bst.search("34"), false);
    assert_eq!(bst.remove(&"2"), false);
    assert_eq!(bst.remove(&"3"), true);
    println!("{:?}", bst);
    assert_eq!(bst.remove(&"6"), true);
    println!("{:?}", bst);
    assert_eq!(bst.remove(&"5"), true);
    println!("{:?}", bst);
    assert_eq!(bst.remove(&"7"), true);
    println!("{:?}", bst);
    assert_eq!(bst.remove(&"4"), true);
    println!("{:?}", bst);
    assert_eq!(bst.remove(&"3"), false);
}
