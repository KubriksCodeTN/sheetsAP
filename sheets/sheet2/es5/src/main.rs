use std::i32;

pub fn max(v: &[i32]) -> Option<i32> {
    if v.len() == 0 { return None; }

    fn max_aux(v: &[i32], i: usize, max: i32) -> i32 {
        if i == v.len() { return  max; }

        return max_aux(v, i + 1, max.max(v[i]));
    }

    return Some(max_aux(v, 1, v[0]));
}

pub fn swap_v(v: &mut [i32]) {
    if v.len() == 0 { return; }

    let l: usize = v.len();
    v.swap(0, l - 1);
}

pub fn is_sorted(v: &[i32]) -> bool {
    if v.len() == 0 { return true; }

    for i in 0..v.len() - 1 {
        if v[i] > v[i + 1] {
            return false;
        }
    }

    return true;
}

pub fn insert_if_longer(v: &mut Vec<String>, s: String) {
    if s.len() > 10 { v.push(s); } // takes ownership
}

fn main() {
    println!("{:?}", max(&Vec::<i32>::new()));
    println!("{:?}", max(&Vec::<i32>::from([1, 2, 3, 4])));
    println!("{:?}", max(&Vec::<i32>::from([1, 5, 3, 4])));
    println!("{:?}", max(&Vec::<i32>::from([4, 4, 4, 4])));
    println!("{:?}", max(&Vec::<i32>::from([4, 8999, 5, 4])));
    println!("{:?}", max(&Vec::<i32>::from([4, 4, 4, 124, 1, 2, -1, -1, -1, 2434534])));
    println!("{:?}", max(&Vec::<i32>::from([-4, -4, -4, -4])));

    println!("{:?}", is_sorted(&[1, 2, 3, 4]));
    println!("{:?}", is_sorted(&[1, 2, 3, 4, 5]));
    println!("{:?}", is_sorted(&[1, 2, 3, 4, 1]));
    println!("{:?}", is_sorted(&[]));
}
