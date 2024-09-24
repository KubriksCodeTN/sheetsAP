// use std::sync::Mutex;

// Write a function multiply that takes an i32 , a f32 and a f64 and returns the multiplication of the
// three of them as a f64 value
fn multiply(a : i32, b : f32, c : f64) -> f64{
    return a as f64 * b as f64 * c;
}

fn main() {
    print!("{}\n{}\n{}\n", multiply(0, 3., 4.), multiply(2, 2.2, 2.12), multiply(-1, -1., -1.1));
}
