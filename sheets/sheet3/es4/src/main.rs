#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Debug)]
enum Item {
    Coffee, 
    Coke,
    Cookie,
}

enum Coin {
    FiftyCents,
    Euro1,
    Euro2,
}

impl Coin {
    fn to_cents(&self) -> u32 {
        match self {
            Coin::FiftyCents => 50u32,
            Coin::Euro1 => 100u32,
            Coin::Euro2 => 200u32,
        }
    }
}

struct VendingMachine {
    coins: u32,
    pub items: HashMap<Item, usize>
}

impl VendingMachine {
    fn new(map: HashMap<Item, usize>) -> VendingMachine {
        return VendingMachine{ coins: 0, items: map };
    }

    fn add_item(&mut self, item: Item, sz: usize) {
        // *self.items.entry(item).or_insert(0) += sz;
        self.items.entry(item).and_modify(|x: &mut usize| *x += sz).or_insert(sz);
    }

    fn insert_coin(&mut self, c: Coin) -> Result<&str, &str> {
        /*
        match self.coins.checked_add(c.to_cents()) {
            Some(_) => Ok("Done"),
            None => Err("Overflow!1!"),
        }
        */

        #[allow(unreachable_patterns)]
        let result: Result<&str, &str> = match c {
            Coin::FiftyCents => Ok("+50 cents"),
            Coin::Euro1 => Ok("+1 euros"),
            Coin::Euro2 => Ok("+2 euros"),
            _ => Err("Invalid Coin"),
        };

        if result.is_ok() {
            self.coins += c.to_cents();
        }

        return result;
    }

    // ???
    fn get_item_price(it: &Item) -> u32 {
        match it {
            Item::Coffee => 100,
            Item::Coke => 200,
            Item::Cookie => 100,
        }
    }

    fn buy(&mut self, it: Item) -> Result<u32, &str> {
        if self.coins < Self::get_item_price(&it) {
            return Err("Not enough money");
        }

        match self.items.get_mut(&it) {
            Some(n) => {
                if *n == 0 {
                    return Err("Item finished");
                }
                *n -= 1;
                self.coins -= Self::get_item_price(&it);
                Ok(self.coins as u32)
            }
            None => Err("Item not available")
        }
    }
}

fn main() {
    let mut v = VendingMachine::new(HashMap::<Item, usize>::new());
    v.add_item(Item::Coffee, 2);
    v.add_item(Item::Coffee, 2);
    println!("{:?}", v.items);
}
