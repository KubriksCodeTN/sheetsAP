#![allow(dead_code)]

use std::collections::HashMap;
use sentence::Sentence;

mod sentence {
    #[derive(Debug)]
    pub struct Sentence {
        pub words: Vec<String>,
    }

    impl Sentence {
        pub fn new_default() -> Sentence {
            return Sentence{ words: Vec::new() };
        }

        pub fn new(s: &str) -> Sentence {
            return Sentence{ words: s.split_whitespace().map(|x: &str| x.to_string()).collect() };
        }
    }
}

// #[cfg(test)] // idk
mod test {
    use std::collections::HashMap;
    use crate::sentence::Sentence;

    pub fn magic_sentence(map: &HashMap<i32, Sentence>, i: i32, j: i32) -> Result<Sentence, &str> {
        let opt1: Option<&Sentence> = map.get(&i);
        let opt2: Option<&Sentence> = map.get(&j);

        if opt1.and(opt2).is_none() {
            return Err("nope");
        }

        let s1: &Vec<String> = &opt1.unwrap().words;
        let s2: &Vec<String> = &opt2.unwrap().words;

        return Ok(Sentence{
            words: s1.iter().zip(s2.iter())
                    .filter_map(|(a, b)| if a == b { Some(a.clone()) } else { None } )
                    .collect()
        });
    }
}

fn main() {
    let mut map: HashMap<i32, Sentence> = HashMap::new();
    map.insert(1, Sentence::new("Hello my name was cool yesterday"));
    map.insert(2, Sentence::new("Hi my name is cool"));
    println!("{:?}", test::magic_sentence(&map, 1, 2));
}
