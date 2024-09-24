// In the main function initialize a HashMap<String, f32> called furniture that stores the pair
// String as key and f32 as value, where the String is the name of the furniture and the f32 is its
// price. Then write a function that borrows the HashMap , takes a furniture: String as input and
// returns the corresponding f32 . If there is no such furniture in the HashMap , return -1.0 ;
use std::collections::HashMap;

// NB consider returning a f32 for lifetime issues (-1.) does not expire
fn get_price(map : &HashMap<String, f32>, s : String) -> &f32 {
    // &String gets converted to &s
    return map.get(&s).unwrap_or(&-1.);
}

fn _f(_s : &String){

}

fn main() {
    let mut forniture : HashMap<String, f32> = HashMap::new();
    forniture.insert("casa".to_string(), 23.);
    println!("{}", get_price(&forniture, "casa".to_string()));
    println!("{}", get_price(&forniture, "c".to_string()));

    let s : String = "hey".to_string();
    let _sp : &str = &s;
    // f(s.as_str());
}
