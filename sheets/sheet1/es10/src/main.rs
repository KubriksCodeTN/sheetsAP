// Write a function that takes a "matrix" (2x2, i32 tuple) as input, transposes and returns it
type Matrix = ((i32, i32,), (i32, i32));

// this is garbage but its funny
fn transpose(((a00, a01), (a10, a11)): Matrix) -> Matrix {
    return ((a00, a10), (a01, a11));
}

fn transpose1(matrix: Matrix) -> Matrix {
    let mut trans: ((i32, i32), (i32, i32)) = matrix;
    let tmp: i32 = trans.0 .1;
    trans.0 .1 = trans.1 .0;
    trans.1 .0 = tmp;
    trans
}

fn main() {
    println!("{:?}", transpose(((1, 2), (1, 2))));
    println!("{:?}", transpose1(((1, 2), (1, 2))));
    println!("{:?}", transpose(((2, 2), (1, 2))));
    println!("{:?}", transpose1(((2, 2), (1, 2))));
}
