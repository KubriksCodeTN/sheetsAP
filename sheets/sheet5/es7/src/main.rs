#![allow(dead_code)]

trait Heatable {
    fn cook(&mut self);
}

trait Friable {
    fn cook(&mut self);
}

trait Heater {
    fn heat(&self, h: &mut dyn Heatable);
}

trait Frier {
    fn fry(&self, f: &mut dyn Friable);
}

struct Oven {}
struct Pan {}

impl Heater for Oven {
    fn heat(&self, h: &mut dyn Heatable) {
        h.cook();
    }
}

impl Heater for Pan {
    fn heat(&self, h: &mut dyn Heatable) {
        h.cook();
    }
}

impl Frier for Pan {
    fn fry(&self, f: &mut dyn Friable) {
        f.cook();
    }
}

struct Pie {
    ready: bool,
}
struct Carrot {
    state: CarrotState,
}

#[derive(PartialEq)]
enum CarrotState {
    Raw,
    Cooked,
    Fried,
    Burnt,
}

trait Edible {
    fn eat(&self);
}

impl Heatable for Pie {
    fn cook(&mut self) {
        if !self.ready {
            self.ready = true;
            return;
        }

        println!("You burned the pie!1!!");
    }
}

impl Heatable for Carrot {
    fn cook(&mut self) {
        if self.state == CarrotState::Raw {
            self.state = CarrotState::Cooked;
            return;
        }

        self.state = CarrotState::Burnt;
    }
}

impl Friable for Carrot {
    fn cook(&mut self) {
        if self.state == CarrotState::Fried {
            self.state = CarrotState::Burnt;
            return;
        }

        self.state = CarrotState::Fried;
    }
}

impl Edible for Pie {
    fn eat(&self) {
        match self.ready {
            false => println!("You died :("),
            true => println!("Damn that's good"),
        }
    }
}

impl Edible for Carrot {
    fn eat(&self) {
        match self.state {
            CarrotState::Burnt => println!("mmh, burnt"),
            CarrotState::Cooked => println!("mmh, yummy"),
            CarrotState::Fried => println!("mmh, crispy"),
            CarrotState::Raw => println!("mmh, crunchy"),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
