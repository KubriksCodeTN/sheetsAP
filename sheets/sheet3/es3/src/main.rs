use std::collections::HashMap;

/*
struct Babbo {
    nome: String,
}

impl Babbo {
    fn add(i: i32, y: i32) -> i32 {
        return i + y;
    }
}
*/

// his solution has a lot of things that are not mentioned in the text???
fn recognize_owner(map: &HashMap<String, String>, owner: String) -> Option<&String> {
    return map.get(&owner);
}

fn main() {
    let mut map: HashMap<String, String> = HashMap::<String, String>::new();

    // Babbo::add(8, 8);
}
