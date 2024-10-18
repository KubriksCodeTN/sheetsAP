trait Printable {
    fn print(&self);
}

impl Printable for i32 {
    fn print(&self) {
        println!("{self}");
    }
}

impl Printable for String {
    fn print(&self) {
        println!("{self}");
    }
}

impl<T: Printable> Printable for Vec<T> {
    fn print(&self) {
        self.iter().for_each(T::print);
    }
}

// we use & cause yes
fn print(e: &impl Printable) {
    e.print();
}

fn main() {
    print(&5);
    print(&"ciao".to_string());
    print(&vec![1, 2, 3]);
}
