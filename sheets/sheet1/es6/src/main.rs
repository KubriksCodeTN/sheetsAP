// Write a function lord_farquaad that takes a String and outputs another String in which every
// character 'e' is substituted by the character '💥';
fn lord_farquaad(s : String) -> String {
    /*
    return s.chars().map(|c : char| {
        if c == 'e' { return '💥' } else { return c };
    }).collect();
    */
    return s.replace('e', "💥");
}

fn main() {
    println!("{}", lord_farquaad("Hello!".to_string())); 
}
