// Write a function bigger that takes two i32 and returns the bigger number ( i32 ) without using
// another function call and additional variables
fn bigger (a : i32, b : i32) -> i32{
    match a > b{
        true => a,
        _ => b,
    }
}

fn main() {
    print!("{}\n{}\n{}\n{}\n", bigger(1, 2), bigger(0, 0), bigger(-2, 1), bigger(-2, -1));
}
