use core::fmt;
// use std::io::Read;

// his implementation uses 3 structs and 3 vectors but this also works fine?
// #[derive(Hash)]
#[allow(dead_code)]
#[derive(Debug)]
enum Readable {
    Book{ name: String, code: u32, year:u32, author: String, company: String },
    Article{ name: String, code: u32, year:u32, orchid: u32 },
    Magazine{ name: String, code: u32, year:u32, number: u32, month: u8 },
}

struct Bup { pub objs: Vec<Readable>, }

// Not gonna format properly
impl fmt::Display for Readable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{:?}", self);
    }
}

impl fmt::Display for Bup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{:?}", self.objs);
    }
}

fn main() {
    let mut bup: Bup = Bup{ objs: Vec::new() };

    bup.objs.push(Readable::Book{ name: "yo".to_string(), code: 32, year: 2022, author: "OOO".to_string(), 
                    company: "bella".to_string()});

    println!("{}", bup);
}
