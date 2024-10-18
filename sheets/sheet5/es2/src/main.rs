#![allow(dead_code)]
use rand::{distributions::{Alphanumeric, DistString}, random};

#[derive(Default, Debug)]
enum Category {
    Fantasy,
    Horror,
    SciFi,
    Romance,
    Thriller,
    Historical,
    Comedy,
    Drama,
    #[default]
    Poetry,
    Other,
}

#[derive(Debug)]
struct Book {
    title: String,
    cat: Category,
}

#[derive(Debug, Default)]
struct Library {
    bookcases: [Vec<Book>; 10],
}

impl Default for Book {
    fn default() -> Self {
        Book { title: Alphanumeric.sample_string(&mut rand::thread_rng(), random::<usize>() % 20),
            cat: Category::default() }
    }
}

impl Book {
    fn default_with_cat(c: Category) -> Self {
        Book { cat: c, ..Self::default() } // * xd *
    }
}

// discombobulate
trait Populatable {
    fn populate(&mut self);
}

impl Populatable for Library {
    fn populate(&mut self) {
        let it = std::iter::repeat_with(|| Book::default());

        // flex (but a simple for is better)
        self.bookcases.iter_mut().for_each(|v: &mut Vec<Book>| {
            v.extend(it.take(3));
        });
    }
}

fn main() {
    let mut l: Library = Library::default();
    l.populate();
    println!("{l:?}");
}
