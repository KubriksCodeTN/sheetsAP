#![allow(dead_code)]

struct Person<'a> {
    name: String,
    father: Option<&'a Person<'a>>,
    mother: Option<&'a Person<'a>>,
}

impl <'a> Person<'a> {
    fn new(name: String, f: Option<&'a Person>, m: Option<&'a Person>) -> Person<'a> {
        Person{ name, father: f, mother: m }
    }

    fn aux_fn<'b>(&'b self, generations: u32, mut v: &mut Vec<&'b Self>) {
        v.push(&self);
        if generations == 0 {
            return;
        }
        if self.father.is_some() {
            self.father.unwrap().aux_fn(generations - 1, &mut v);
        }
        if self.mother.is_some() {
            self.mother.unwrap().aux_fn(generations - 1, &mut v);
        }
    }

    fn find_relatives(&self, generations: u32) -> Vec<&Self> {
        let mut v: Vec<&Self> = Vec::new();
        self.aux_fn(generations, &mut v);
        v
    }

    fn aux_fn2<'b>(&'b self, mut v: &mut Vec<&'b Self>) {
        match (self.father, self.mother) {
            (None, None) => {
                v.push(&self);
            }
            (Some(p), None) => {
                v.push(&self);
                p.aux_fn2(&mut v);
            }
            (None, Some(p)) => {
                v.push(&self);
                p.aux_fn2(&mut v);
            }
            (Some(p1), Some(p2)) => {
                p1.aux_fn2(&mut v);
                p2.aux_fn2(&mut v);
            }
        }
    }

    fn find_roots(&self) -> Vec<&Self> {
        let mut v: Vec<&Self> = Vec::new();
        self.aux_fn2(&mut v);
        v
    }
}

fn main() {
    
}

/*
struct Point<'a> {
    x: &'a i32,
    y: &'a i32,
}

fn main() {
    let x1 = 1;
    let v;
    {
        let y1 = 2;
        let f: Point<'_> = Point { x: &x1, y: &y1 };
        v = f.x;
    }
    println!("{}", *v); // no cause infers lifetime of y
}
 */