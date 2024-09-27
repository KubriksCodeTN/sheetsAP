#![allow(dead_code, unused_imports)]

pub struct X {
    s: Option<String>,
    i: i32,
}

impl X {
    pub fn new(s: &str, i: i32) -> X {
        return X{ s: Some(s.to_string()), i  };
    }

    pub fn take_str(&mut self) -> Option<String> {
        let tmp: Option<String> = self.s.take();
        self.s = None;
        return tmp;
    }
}

/* */

pub struct NameSurname {
    name: String,
    surname: String,
}

pub fn replace_surname(mut ns: NameSurname, mut s: String) -> String {
    std::mem::swap(&mut ns.surname, &mut s);
    return s;
}

/* */

#[derive(Debug, Clone)]
pub struct Student {
    name: String,
    id: u32,
}

impl Student {
    pub fn new(s: &str, id: u32) -> Student {
        return Student{ name: s.to_string(), id };
    }
}

impl std::fmt::Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{:?}", self);
    }
}

pub struct University {
    name: String,
    students: Vec<Student>,
}

impl University {
    pub fn new(s: &str, v: &[Student]) -> University {
        return University{ name: s.to_string(), students: v.to_vec() };
    }

    pub fn remove_student(&mut self, id: u32) -> Result<Student, &str> {
        for (i, s) in self.students.iter_mut().enumerate() {
            if s.id == id {
                return Ok(self.students.remove(i));
            }
        }
        return Err("Not found");
    }
}

impl std::fmt::Display for University {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}\nStudents: {:?}", self.name, self.students);
    }
}

pub fn prev_char(c: char) -> char {
    if c.is_ascii_alphabetic() && c != 'A' && c != 'a' {
        return (c as u8 - 1) as char;
    }
    return c;
}

pub fn prev_str(s: &str) -> String {
    return s.chars().map(|x: char| return prev_char(x)).collect();
}

/* */
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum AirplaneCompany {
    Boeing,
    Airbus,
}

#[derive(Clone, Debug)]
pub struct Airplane {
    pub company: AirplaneCompany,
    pub model: String,
}

#[derive(Debug)]
pub struct AirFleet {
    pub fleet: Vec<Airplane>,
}

impl AirFleet {
    pub fn remove_boeing(&mut self) {
        self.fleet = self.fleet.to_owned()
                        .into_iter()
                        .filter(|x: &Airplane| x.company != AirplaneCompany::Boeing)
                        .collect();
    }

    pub fn add_airplane(&mut self, a: Airplane) {
        self.fleet.push(a);
    }

    pub fn search_airplane(&self, model: &str) -> Result<AirplaneCompany, String> {
        for a in &self.fleet {
            if a.model == model {
                return Ok(a.company);
            }
        }

        return Err("Not in this fleet".to_string());
    }
}

/* */
use std::collections::HashMap;
// use crate::unumber::Unumber;
use crate::hashmaps::Maps;
use crate::module::*;

pub mod unumber {
    pub type Unumber = usize;
}

pub mod hashmaps {
    use std::collections::HashMap;
    use crate::unumber::Unumber;

    pub struct Maps {
        pub map: HashMap<Unumber, String>
    }
}

pub mod module {
    use std::collections::HashMap;

    use crate::unumber::Unumber;
    use crate::hashmaps::Maps;

    pub fn string_to_tuple(m: Maps) -> HashMap<Unumber, (Unumber, String)> {
        let mut toret: HashMap<Unumber, (Unumber, String)> = HashMap::new();

        for (k, v) in m.map {
            toret.insert(k, (v.len(), v));
        }

        return toret;
    }
}

/* */
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Size {
        return Size{ width, height }; 
    }

    pub fn area(&self) -> f32 {
        return self.width * self.height;
    }

    pub fn compare(&self, s: &Size) -> Option<bool> {
        if self.area() == s.area() {
            return None;
        }
        else if self.area() > s.area() {
            return Some(true);
        }
        
        return Some(false);
    }
}

/* */
struct MaybePoint {
    x: Option<i32>,
    y: Option<i32>,
}

impl MaybePoint {
    pub fn new(x: Option<i32>, y: Option<i32>) -> MaybePoint {
        return MaybePoint{ x, y };
    }

    pub fn is_some(&self) -> bool {
        return self.x.and(self.y).is_some();
    }

    pub fn maybe_len(&self) -> Option<f32> {
        let x: i32 = self.x?;
        let y: i32 = self.y?;
        return Some(
            (((x * x) + (y * y)) as f32).sqrt()
        );
    }
}

/* */
pub fn res1(x: i32) -> Result<i32, String> {
    return if x % 10 == 0 { Ok(x) } else { Err("error".to_string()) };
}

// why &str!?!?
pub fn res2(r: Result<i32, &str>) -> Result<i32, String> {
    r?;
    return if r.unwrap() % 5 == 0 { Ok(r.unwrap()) } else { Err("error".to_string()) };
}

pub fn wrapper(x: i32) -> Result<i32, String> {
    return res2(Ok(res1(x)?));
}

/* */
pub fn order(v: Vec<String>) -> Vec<String> {
    let mut v1: Vec<String> = Vec::new();

    for (i, s) in v.iter().enumerate() {
        let mut s1: String = i.to_string();
        s1.push_str(" - ");
        s1.push_str(s);
        v1.push(s1);
    }

    return v1;
}

/* */
pub mod modx {
    pub enum X {
        S(char),
        C(String),
    }
}

pub mod mody {
    pub enum X {
        F(f64, usize),
    }
}

pub mod modsum {
    use super::modx;
    use super::mody;

    pub fn sum(x: modx::X, y: mody::X) -> f64 {
        let mody::X::F(f, u) = y;

        match x {
            modx::X::C(c) => c.len() as f64 + f * u as f64,
            modx::X::S(s) => s as u8 as f64 + f * u as f64,
        }
    }
}

pub fn main() {
}
