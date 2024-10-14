#![allow(dead_code)]

struct Chair<'a> {
    color: &'a str,
    quantity: &'a usize,
}

struct Wardrobe<'a> {
    color: &'a str,
    quantity: &'a usize,
}

trait Object {
    fn build(&self) -> &str;
    fn get_quantity(&self) -> String;
}

impl<'a> Object for Chair<'a> {
    fn build(&self) -> &str {
        "Chair has been built!"
    }

    fn get_quantity(&self) -> String {
        format!("{} chairs", self.quantity)
    }
}

impl<'a> Object for Wardrobe<'a> {
    fn build(&self) -> &str {
        "Wardrobe has been built!"
    }

    fn get_quantity(&self) -> String {
        format!("{} wardrobes", self.quantity)
    }
}

impl<'a> std::fmt::Display for Chair<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.quantity {
            0 => write!(f, "No chairs!"),
            _ => write!(f, "{} {} chairs!", self.quantity, self.color),
        }
    }
}

impl<'a> std::fmt::Display for Wardrobe<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.quantity {
            0 => write!(f, "No wardrobes!"),
            _ => write!(f, "{} {} wardrobes!", self.quantity, self.color),
        }
    }
}

fn main() {
    let w: Wardrobe<'_> = Wardrobe { color: "c", quantity: &5 };
    println!("{}", w.get_quantity());
    println!("{w}");
    println!("{}", w.get_quantity());
}
