use std::collections::HashMap;

pub fn count_chars(s: &str) -> HashMap<char, u32> {
    let mut h: HashMap<char, u32> = HashMap::new();

    for c in s.chars() {
        *h.entry(c).or_insert(0) += 1;
    }

    return h;
}

fn main() {
    println!("{:?}", count_chars("ciao bello mio"));
}
