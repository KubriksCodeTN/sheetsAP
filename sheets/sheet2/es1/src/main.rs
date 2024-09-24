pub fn modify_odd(s: &mut [i32]) {
    s.iter_mut().for_each(|x: &mut i32| if *x % 2 == 1 { *x = 0 });
}

pub fn get_100() -> Vec<i32> {
    return (0..101).collect();
}

fn main() {
    let mut v: Vec<i32> = get_100();
    modify_odd(&mut v);
    println!("{:?}", v);
}
