pub fn merge(v1: &[i32], v2: &[i32]) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();

    let mut l1: usize = 0;
    let mut l2: usize = 0;

    while l1 < v1.len() && l2 < v2.len() {
        if v1[l1] < v2[l2] {
            v.push(v1[l1]);
            l1 += 1;
        }
        else {
            v.push(v2[l2]);
            l2 += 1;
        }
    }

    for e in &v1[l1..] {
        v.push(*e);
    }

    for e in &v2[l2..] {
        v.push(*e);
    }

    return v;
}

fn main() {
    println!("{:?}", merge(&[1, 2, 3], &[1, 2, 3]));
    println!("{:?}", merge(&[1, 2, 3], &[3, 3, 3]));
    println!("{:?}", merge(&[1, 2, 3], &[3, 4, 5]));
    println!("{:?}", merge(&[1, 2, 3], &[]));
    println!("{:?}", merge(&[], &[1, 2, 3]));
    println!("{:?}", merge(&[], &[]));
    println!("{:?}", merge(&[4, 5, 6], &[1, 2, 3]));
}
