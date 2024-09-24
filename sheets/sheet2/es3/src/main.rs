pub fn split_at_val(s: &[i32], v: i32) -> Option<(&[i32], &[i32])> {
    return Some(s.split_at(s.iter().position(|x: &i32| *x == v)?));
}

fn main() {
    println!("{:?}", split_at_val(&[1, 2, 3, 4, 5], 2));
    println!("{:?}", split_at_val(&[1, 2, 3, 4, 5], 45));
    println!("{:?}", split_at_val(&[1, 2, 3, 4, 5], 1));
    println!("{:?}", split_at_val(&[1, 2, 3, 4, 5], 5));
    println!("{:?}", split_at_val(&[1, 2, 3, 4, 5], -1));
}
