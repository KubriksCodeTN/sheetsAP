use std::fmt::{Debug, Display};

fn restricted<T, U>(t1: T, t2: T, u: U) -> T
    where T: Ord + PartialOrd + Debug,
          U: Display {
    let min = t1.min(t2);
    print!("minor: <{min:?}>\nu: <{u}>\n");
    min
}

fn main() {
    println!("{}", restricted(2, 4, "ciao"));
}
