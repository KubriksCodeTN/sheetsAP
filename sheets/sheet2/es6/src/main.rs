use std::slice::Iter;

pub fn build_vector(it: Iter<i32>) -> Vec<&i32> {
    return it.collect();
}

fn main() {
    println!("Hello, world!");
}
