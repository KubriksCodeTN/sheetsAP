#![allow(dead_code)]
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct EntangledBit {
    bit: Rc<RefCell<bool>>,
}

impl std::default::Default for EntangledBit {
    fn default() -> Self {
        Self { bit: Rc::new(RefCell::new(false)) }
    }
}

impl EntangledBit {
    fn get(&self) -> bool {
        *self.bit.borrow()
    }

    fn set(&self) {
        *self.bit.borrow_mut() = true;
    }

    fn reset(&self) {
        *self.bit.borrow_mut() = false;
    }

    fn entangle(&self, o: &mut EntangledBit) {
        o.bit = self.bit.clone();
    }
}

// This doesn't work, can't entangle more than 2 people at a time (:
fn main() {
    let mut b1 = EntangledBit::default();
    let mut b2 = EntangledBit::default();
    let b3 = EntangledBit::default();

    b1.set();
    b1.entangle(&mut b2);
    b3.entangle(&mut b1);
    b1.set(); // b3.set();
    b2.reset();

    println!("{:?} {:?} {:?}", b1, b2, b3);
}
