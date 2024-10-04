#![allow(dead_code)]
// #![derive(Debug)]

use std::collections::HashMap;

#[derive(Debug)]
enum A {
    A2(char, char),
    A1(i32, i32 , i32),
}

#[derive(Debug)]
enum B {
    B1(i32, i32),
    B2(String),
}

fn bfroma(a: A) -> B {
    match a {
        A::A1(a, b, c) => {
            let mut s: String = String::new();
            s.push_str(&a.to_string());
            s.push_str("-");
            s.push_str(&b.to_string());
            s.push_str("-");
            s.push_str(&c.to_string());
            B::B2(s)
        }
        A::A2(c, d) => B::B1(c as i32, d as i32)
    }
}

/* */

#[derive(Debug)]
enum E {
    A(String),
    B(bool),
}

#[derive(Debug)]
enum F {
    F1(String),
    F2(i32),
}

impl E {
    pub fn count_vowels(&self) -> usize {
        match self {
            E::B(_) => 0,
            E::A(s) => {
                let mut sum: usize = 0;
                for c in s.chars() {
                    if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                        sum += 1;
                    }
                } 
                sum
            }
        }
    }
}

impl F {
    pub fn new() -> F {
        return F::F1("hello".to_string());
    }

    pub fn calculation(&self) -> usize {
        match self {
            F::F1(s) => s.len(),
            F::F2(n) => *n as usize,
        }
    }
}

/* */

fn print_n(op: Option<i32>) {
    match op {
        Some(x) => {
            for _ in 0..x {
                println!("{}", x);
            }
        }
        _ => { println!("Error"); }
    }
}

/* */

struct Balance {
    amt: i32,
    active: bool,
}

impl Balance {
    pub fn maybericher(&self, b: Balance) -> Option<bool> {
        if !self.active || !b.active {
            return None;
        }

        return Some(self.amt > b.amt);
    }
}

/* */

struct G {
    x: i32, 
    y: i32,
}

impl G {
    pub fn new(x: i32, y: i32) -> G {
        return G{ x, y };
    }

    pub fn square(&self) -> Result<i32, ()> {
        return if self.y * self.y == self.x { Ok(self.x) } else { Err(()) };
    }
}

/* */

#[derive(Debug)]
struct X {
    s: String,
    i: i32,
}

#[derive(Debug)]
struct Y {
    b: bool,
    c: String,
}

impl X {
    pub fn new() -> X {
        return X{ s: "xxx".to_string(), i: 10 };
    }

    pub fn getstr(&mut self) -> String {
        return std::mem::take(&mut self.s);
    }
}

impl Y {
    pub fn new() -> Y {
        return Y{ b: true, c: "op".to_string() };
    }

    pub fn getstr(&mut self) -> String {
        return std::mem::take(&mut self.c);
    }
}

impl std::fmt::Display for Y {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "B {}, C {}", self.b, self.c)
    }
}

impl std::fmt::Display for X {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "S {}, I {}", self.s, self.i)
    }
}

fn swapstr(mut x: X, mut y: Y) -> (X, Y) {
    std::mem::swap(&mut x.s, &mut y.c);
    return (x, y);
}

/* */

#[derive(Debug)]
struct L {
    s: String,
    n: i32,
}

#[derive(Debug)]
struct M {
    s: String,
    n: f64,
}

impl L {
    pub fn new() -> L {
        return L{ s: "hello".to_string(), n: 0 };
    }

    pub fn new_with_params(s: String, n: i32) -> L {
        return L{ s, n };
    }
}

impl M {
    pub fn new() -> M {
        return M{ s: "hello".to_string(), n: 0. };
    }

    pub fn new_with_params(s: String, n: f64) -> M {
        return M{ s, n };
    }
}

fn swap_string(l: &mut L, m: &mut M) {
    std::mem::swap(&mut l.s, &mut m.s);
}

/* */

fn neighbour(v: &Vec<String>, i: usize) -> Result<String, ()> {
    match v.get(i) {
        Some(s) => {
            match v.get(i + 1) {
                Some(s1) => {
                    let mut s2 = s.clone();
                    s2.push_str(&s1);
                    return Ok(s2);
                }
                None => Err(())
            }
        },
        None => return Err(()),
    }
}

/* */

fn removeelement(v: &mut Vec<Option<i32>>) {
    let i: Option<usize> = v.iter().position(|x: &Option<i32>| {
        match x {
            Some(i) => return i % 2 == 1,
            None => true,
        }
    });

    if i.is_some() {
        v.remove(i.unwrap());
    }
}

/* */

fn hashandhash(h1: &mut HashMap<i32, String>, h2: &mut HashMap<String, i32>) {
    *h2 = h2.clone().into_iter().filter(|(k, _)| return h1.get(&(k.len() as i32)).is_none()).collect();
}

/* */

fn unique(mut h: HashMap<i32, String>, i: i32) -> Option<HashMap<i32, String>> {
    if i < 0 {
        return None;
    }

    for (_k, v) in &h {
        if v.len() == i as usize {
            return None;
        }
    }

    h.insert(h.len() as i32, "X".repeat(i as usize));
    return Some(h);
}

fn main() {
}