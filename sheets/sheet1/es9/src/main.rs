/* An Armstrong number is a number that is the sum of its own digits each raised to the power of the
number of digits.
For example:
9 is an Armstrong number, because 9 = 9^1 = 9
10 is not an Armstrong number, because 10 ≠ 1^2 + 0^2 = 1
153 is an Armstrong number, because: 153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153
154 is not an Armstrong number, because: 154 ≠ 1^3 + 5^3 + 4^3 = 1 + 125 + 64 = 190
Write the function is_armstrong that determines whether a number is an Armstrong number; */
fn is_armstrong(n: i32) -> bool {
    let mut n1: i32 = n;
    let mut v: Vec<i32> = Vec::new();

    while n1 != 0 {
        v.push(n1 % 10);
        n1 /= 10;
    }

    print!("{}\n", v.len());
    return n == v.iter().fold(0, |acc: i32, x: &i32| -> i32 { acc + x.pow(v.len() as u32) });
}

fn main() {
    println!("{}", is_armstrong(0));
    println!("{}", is_armstrong(9));
    println!("{}", is_armstrong(10));
    println!("{}", is_armstrong(153));
    println!("{}", is_armstrong(154));
}
