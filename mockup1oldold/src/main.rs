#![allow(dead_code)]

enum XX {
    Y1(i32, i32),
    Y2(String),
}

fn data(x: &XX) -> i32 {
    match x {
        XX::Y1(a, b) => a * b,
        XX::Y2(s) => s.len() as i32,
    }
}

/* */

enum Z {
    Y1(i32, i32),
    Y2(bool, String),
}

fn maybelength(z: &Z) -> Option<usize> {
    match z {
        Z::Y1(_, _) => None,
        Z::Y2(_, s) => Some(s.len()),
    }
}

/* */

mod enumx {
    pub enum X {
        Y(String),
    }
}

mod structx {
    pub struct X {
        pub i: String,
    }
}

mod modfun {
    pub fn longer(x1: &super::enumx::X, x2: &super::structx::X) -> usize {
        let super::enumx::X::Y(x) = x1; 
        return x.len().max(x2.i.len());
        
    }
}

/* */

fn maybesqrt(i: i32) -> Option<i32> {
    if i < 0 {
        return None;
    }
    return Some((i as f32).sqrt() as i32);
}

/* */

struct Balance {
    amt: i32,
}

impl Balance {
    pub fn withdraw(&self, n: i32) -> Option<i32> {
        if self.amt < n {
            return None;
        }

        return Some(self.amt - n);
    }
}

/* */

fn prevchar(c: char) -> char {
    return (c as i32 - 1) as u8 as char;
}
 
fn replwithprev(s: &mut String) -> Result<String, ()> {
    if s.contains("a") {
        return Err(());
    }

    return Ok(s.chars().map(|c| prevchar(c) as u8 as char).collect());
}

/* */

#[derive(Debug)]
struct X {
    s: String,
    i: i32,
}

#[derive(Debug)]
struct Y {
    z: bool,
    c: String,
}

impl X {
    pub fn new() -> X {
        return X{ s: "xxx".to_string(), i: 10 };
    }

    pub fn getstring(&mut self) -> String {
        return std::mem::take(&mut self.s);
    }
}

impl Y {
    pub fn new() -> Y {
        return Y{ z: true, c: "op".to_string() };
    }

    pub fn getstring(&mut self) -> String {
        return std::mem::take(&mut self.c);
    }
}

impl std::fmt::Display for X {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "S {}, I {}", self.s, self.i)
    }
}

impl std::fmt::Display for Y {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "B {}, C {}", self.z, self.c)
    }
}

fn swapstr(mut x: X, mut y: Y) -> (X, Y){
    std::mem::swap(&mut x.s, &mut y.c);
    (x, y)
}

/* */

#[derive(Debug)]
enum C {
    C1(i32, i32),
    C2(bool, String),
}

#[derive(Debug)]
struct D {
    c: C,
    s: String,
}

impl D {
    pub fn new() -> D {
        return D{ c: C::C1(0, 0), s: "".to_string() };
    }

    #[allow(non_snake_case)]
    pub fn new_with_C(c: C) -> D {
        match c {
            C::C1(_, _) => D{ c, s: "not there".to_string() },
            C::C2(z, s) => D{ c: C::C2(z, s.clone()), s},
        }
    }

    pub fn larger(&self) -> usize {
        match self.c {
            C::C1(_, _) => self.s.len(),
            C::C2(_, ref s) => self.s.len().max(s.len()),
        }
    }
}

impl std::fmt::Display for D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "D: {} with {:?}", self.s, self.c)
    }
}

/* */

fn swapelconcat(v: &mut Vec<String>, i: usize, j: usize) -> Option<&Vec<String>> {
    let s1 = v.get_mut(i)?.clone();
    let s2 = v.get_mut(j)?.clone();
    v[i].push_str(&s2);
    v[j].push_str(&s1);
    return Some(v);
}

/* */

fn veclengths(v: &Vec<String>) -> Vec<i32> {
    return v.iter().map(|s| s.len() as i32).collect();
}

/* */

fn removeelfromvector(v: &mut Vec<String>, n: usize) {
    for i in 0..v.len() {
        if v[i].len() == n {
            v.remove(i);
            return;
        }
    }
}

/* */

mod test_mt_2022 {
    #[allow(unused_imports)]
    use super::*;
    
    #[test]
    fn test_ex1() {
        let xx1 = XX::Y1(2,3);
        assert_eq!(data(&xx1), 6);
        let xx1 = XX::Y2(String::from("test"));
        assert_eq!(data(&xx1), 4);
    }

    #[test]
    fn test_ex2() {
        let z1 = Z::Y1(1,2);
        assert_eq!(maybelength(&z1), None);
        let z2 = Z::Y2(true, String::from("new"));
        assert_eq!(maybelength(&z2), Some(3));
    }

    #[test]
    fn test_ex3() {
        let ex = enumx::X::Y(String::from("test"));
        let sx = structx::X{i:String::from("asd")};
        assert_eq!(modfun::longer(&ex, &sx), 4);
        let ex = enumx::X::Y(String::from("asdasd"));
        assert_eq!(modfun::longer(&ex,&sx), 6);
    }

    #[test]
    fn test_ex4() {
        assert_eq!(maybesqrt(4), Some(2));
        assert_eq!(maybesqrt(-4), None);
    }

    #[test]
    fn test_ex5() {
        let b = Balance{amt: 100};
        assert_eq!(b.withdraw(10), Some(90));
        assert_eq!(b.withdraw(200), None);
    }

    #[test]
    fn test_ex6() {
        assert_eq!(prevchar('c'), 'b');
        assert_eq!(prevchar('a'), '`');
        assert_eq!(prevchar('z'), 'y');
        let mut s = String::from("Pign");
        assert_eq!(replwithprev(&mut s), Ok("Ohfm".to_string()));
        let mut s1 = String::from("pigna");
        assert_eq!(replwithprev(&mut s1), Err(()));
    }

    #[test]
    fn test_ex7() {
        let x = X::new();
        let y = Y::new();
        // println!("X {:?} - Y {:?}", x ,y);
        let (_x, _y) = swapstr(x, y);
        // println!("X {} - Y {}", x, y);
    }

    #[test]
    fn test_ex8() {
        let c1 = C::C1(0,1);
        let c2 = C::C2(true, String::from("no way jose"));
        let d1 = D::new_with_C(c1);
        let d2 = D::new_with_C(c2);
        assert_eq!(d1.larger(), 9);
        assert_eq!(d2.larger(), 11);
    }

    #[test]
    fn test_ex17() {
        let mut v: Vec<String> = vec![String::from("what");4];
        v.push(String::from("now"));    v.push(String::from("what"));
        println!("{:?}",v);
        swapelconcat(&mut v, 0, 0);
        println!("{:?}",v);

        let mut v: Vec<String> = vec![String::from("what");4];
        v.push(String::from("now"));    v.push(String::from("what"));
        println!("{:?}",v);
        swapelconcat(&mut v, 10000, 0);
        println!("{:?}",v);

        let mut v: Vec<String> = vec![String::from("what");4];
        v.push(String::from("now"));    v.push(String::from("what"));
        println!("{:?}",v);
        swapelconcat(&mut v, 2, 0);
        println!("{:?}",v);
    }

    #[test]
    fn test_ex9() {
        let v1 = vec![String::from("some"); 12];
        let result = veclengths(&v1);
        assert_eq!(result.len(), 12);
        for l in result.iter() {
            assert_eq!(*l, 4);
        }

    }

    #[test]
    fn test_ex10() {
        let mut v = vec![String::from("what"); 5];
        v.push(String::from("now"));
        removeelfromvector(&mut v, 3);
        assert_eq!(v.len(), 5);
        for s in v.iter() {
            assert_eq!(*s, "what".to_string());
        }
    }

}

/* */

pub fn main(){
    let mut v: Vec<String> = vec![String::from("what");4];
    v.push(String::from("now"));    v.push(String::from("what"));
    println!("{:?}",v);
    swapelconcat(&mut v, 0, 0);
    println!("{:?}",v);
}