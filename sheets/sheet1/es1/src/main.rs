// Write a function string_reverse that takes a &str as input and returns it, reversed as a String 
extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

fn string_reverse(s : &str) -> String{
    return s.graphemes(true).rev().collect();
    // return s.chars().rev().collect(); // ASCII only
}

fn main() {
    /*
    let s : String = String::from("Hey!");
    let hey = &s[..3]; // no & is weird
    */
    println!("{}", string_reverse("Åström"));

    /*
    let s: String = String::from("yo");
    // let t: std::str::Chars<'_> = s.chars(); // iterator
    let t: &[u8] = s.as_bytes();
    for &c in t{ // this might be cleaner
        print!("{c}");
    }
    print!("{t:?}");
    */
}
