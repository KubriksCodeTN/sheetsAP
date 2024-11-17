#![allow(dead_code, unused_imports)]
use std::{arch::x86_64::_MM_EXCEPT_UNDERFLOW, cmp::Ordering, f32::consts::E, io::Empty};

#[derive(Debug)]
enum Bst<T: Ord> {
    Node(T, Box<Bst<T>>, Box<Bst<T>>),
    Empty,
}

impl<T: Ord> Bst<T> {
    fn new() -> Self {
        Self::Empty
    }

    fn add(&mut self, elem: T) {
        match self {
            Self::Empty => *self = Self::Node(elem, Box::new(Self::Empty), Box::new(Self::Empty)),
            Self::Node(e, l, r) => {
                match elem.cmp(e) {
                    Ordering::Greater => Self::add(r, elem),
                    _ => Self::add(l, elem),
                }
            }
        }
    }

    fn search(&self, elem: &T) -> bool {
        match self {
            Self::Empty => false,
            Self::Node(e, l, r) => {
                match elem.cmp(e) {
                    Ordering::Equal => true,
                    Ordering::Less => l.search(elem),
                    Ordering::Greater => r.search(elem),
                }
            }
        }
    }

    fn get_min_above(&mut self) -> T {
        match self {
            Self::Empty => panic!(),
            Self::Node(_, l, r) => {
                match &mut **l {
                    Self::Empty => {
                        let tmp = std::mem::replace(&mut **r, Self::Empty);
                        let tmp = std::mem::replace(self, tmp);
                        match tmp {
                            Self::Empty => panic!(),
                            Self::Node(e, _, _) => {
                                e
                            }
                        }
                    }
                    Self::Node(_, _, _) => {
                        l.get_min_above()
                    }
                }
            }
        }
    }

    fn remove(&mut self, elem: &T) -> bool {
        match self {
            Self::Empty => false,
            Self::Node(e, l, r) => {
                match elem.cmp(e) {
                    Ordering::Equal => {
                        match (&mut **l, &mut **r) {
                            (Self::Node(_, _, _), Self::Node(_, _, _)) => {
                                *e = r.get_min_above();
                                return true;
                            }
                            (Self::Node(_, _, _), Self::Empty) => {
                                let tmp = std::mem::replace(&mut **l, Self::Empty);
                                *self = tmp;
                                return true;
                            }
                            (_, _) => {
                                let tmp = std::mem::replace(&mut **r, Self::Empty);
                                *self = tmp;
                                return true;
                            }
                        }
                    }
                    Ordering::Less => l.remove(elem),
                    Ordering::Greater => r.remove(elem)
                }
            }
        }
    }
}

fn main() {
    let mut bst: Bst<&str> = Bst::new();
    bst.add("5");
    bst.add("3");
    bst.add("6");
    bst.add("4");
    bst.add("7");
    println!("{:?}", bst);
    assert_eq!(bst.search(&"7"), true);
    assert_eq!(bst.search(&"3"), true);
    assert_eq!(bst.search(&"33"), false);
    assert_eq!(bst.search(&"34"), false);
    assert_eq!(bst.remove(&"2"), false);
    assert_eq!(bst.remove(&"3"), true);
    println!("{:?}", bst);
    assert_eq!(bst.remove(&"6"), true);
    println!("{:?}", bst);
    bst.add("6");
    assert_eq!(bst.remove(&"5"), true);
    println!("{:?}", bst);
    assert_eq!(bst.remove(&"7"), true);
    println!("{:?}", bst);
    assert_eq!(bst.remove(&"4"), true);
    println!("{:?}", bst);
    assert_eq!(bst.remove(&"3"), false);
}
