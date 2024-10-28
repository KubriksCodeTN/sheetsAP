trait Sound {
    fn make_sound(&self) -> String;
}

struct Cat;
struct Dog;
struct Cow;
struct Frog;

impl Sound for Cat {
    fn make_sound(&self) -> String {
        format!("meow meow")
    }
}

impl Sound for Dog {
    fn make_sound(&self) -> String {
        format!("woof woof")
    }
}

impl Sound for Cow {
    fn make_sound(&self) -> String {
        format!("moo moo")
    }
}

impl Sound for Frog {
    fn make_sound(&self) -> String {
        format!("croak croak")
    }
}

struct FarmCell {
    element: Box<dyn Sound>,
    next: Option<Box<FarmCell>>,
}

impl FarmCell {
    fn new(element: Box<dyn Sound>) -> Self {
        FarmCell { element, next: None }
    }

    /*
    fn insert(&mut self, mut e: Box<dyn Sound>) {
        std::mem::swap(&mut self.element, &mut e);
        let mut new: FarmCell = FarmCell::new(e);
        new.next = self.next.take();
        self.next = Some(Box::new(new));
    }
     */

    fn insert(&mut self, e: Box<dyn Sound>) {
        match &mut self.next {
            Some(n) => n.insert(e),
            None => self.next = Some(Box::new(FarmCell::new(e))),
        }
    }
}

impl Sound for FarmCell {
    fn make_sound(&self) -> String {
        match &self.next {
            Some(n) => self.element.make_sound() + " " + &n.make_sound(),
            None => self.element.make_sound(),
        }
    }
}

#[test]
fn test_list() {
    let mut list = FarmCell::new(Box::new(Dog));
    list.insert(Box::new(Cat));
    list.insert(Box::new(Frog));
    list.insert(Box::new(Cow));
    // println!("{}", list.make_sound());
    assert_eq!(list.make_sound(), "woof woof meow meow croak croak moo moo");   
}

fn main() {
    println!("Hello, world!");
}
