#![allow(dead_code)]

use std::collections::LinkedList;

trait Split<'a> {
    type ReturnType;
    
    fn split(&'a self) -> (Self::ReturnType, Self::ReturnType);
}

impl<'a> Split<'a> for String {
    type ReturnType = &'a str;

    fn split(&'a self) -> (Self::ReturnType, Self::ReturnType) {
        self.as_str().split_at(self.len() / 2)
    }
}

impl<'a> Split<'a> for &[i32] {
    type ReturnType = &'a [i32];

    fn split(&'a self) -> (Self::ReturnType, Self::ReturnType) {
        self.split_at(self.len() / 2)
    }
}

impl<'a> Split<'a> for LinkedList<f64> {
    type ReturnType = LinkedList<f64>;

    fn split(&'a self) -> (Self::ReturnType, Self::ReturnType) {
        let mut tmp: LinkedList<f64> = self.clone();
        let split: LinkedList<f64> = tmp.split_off(self.len() / 2);
        (tmp, split)
    }
}

fn main() {
    println!("Hello, world!");
}
