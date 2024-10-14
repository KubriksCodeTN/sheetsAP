#![allow(dead_code)]

struct DoubleRef<'a, 'b: 'a, T> {
    r: &'a T,
    s: &'b T
}

fn main() {
    let i: &String = &String::from("hey");
    let tmp: &String;
    {
        let d: DoubleRef<String>;
        let j: &String = &String::from("hey");
        d = DoubleRef{ r: j, s: i };
        tmp = d.s; // no r cause dropped
    }
    println!("{tmp}");
}
