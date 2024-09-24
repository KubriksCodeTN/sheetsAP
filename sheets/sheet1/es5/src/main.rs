// Given a vector of i32 , create a function max_min that returns the maximum and the minimum value
// inside that vector
fn max_min(v : Vec<i32>) -> Option<(i32, i32)>{
    if v.is_empty(){
        return None; // idk
    }

    return Some(v.iter().fold((i32::MIN, i32::MAX), |(max, min) : (i32, i32), x : &i32| -> (i32, i32){
        return (Ord::max(max, *x), Ord::min(min, *x));
    }))
}

fn main() {
    let v : Vec<i32> = vec![1, 2, 3, 4];
    println!("{:?}", max_min(v));
    // println!("{v:?}"); moved
    println!("{:?}", max_min(vec![-1, -2, -3, -4]));
    println!("{:?}", max_min(vec![-1, -2, 3, 4]));
}
