fn flip(v: &mut Vec<i32>, mut k: usize) {
    let mut left: usize = 0;

    while left < k {
        v.swap(left, k);
        k -= 1;
        left += 1;
    }
}

fn max_i(v: &mut Vec<i32>, k: usize) -> usize {
    let mut i: usize = 0;

    for j in 0..k {
        if v[j] > v[i] {
            i = j;
        }
    }

    return i;
}

pub fn pancake_sort(v: &mut Vec<i32>) -> &mut Vec<i32> {
    let mut n: usize = v.len();

    while n > 1 {
        let maxi: usize = max_i(v, n);
        if maxi != n - 1 {
            if maxi != 0 {
                flip(v, maxi);
            }
            flip(v, n - 1);
        }
        n -= 1;
    }

    return v;
}

fn main() {
    println!("{:?}", pancake_sort(&mut vec![1, 4, 5, 6, 7, 7, 1]));
    println!("{:?}", pancake_sort(&mut Vec::<i32>::new()));
    println!("{:?}", pancake_sort(&mut vec![1, -1, 1, -1, 1, -1]));
    println!("{:?}", pancake_sort(&mut vec![1, 4, 5, 6, 7, 7]));
}
