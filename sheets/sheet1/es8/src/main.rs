/* Write a function append that takes a String , appends the word "foobar" to it and returns it.
Then write a main function in which you:
- Declare a String initialized with some text.;
- Pass the String to the function append ;
- Print the original String and the one returned by append ;
(do it in this order!) */
fn append(mut s : String) -> String {
    s.push_str("foobar");
    return s;
}

fn main() {
    let s : String = String::from("Ciao ");
    let s1 : String = append(s.clone());
    println!("{s}");
    println!("{s1}");
}
