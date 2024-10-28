#![allow(dead_code)]
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Car {
    model: String,
    year: u32,
    price: u32,
    rent: bool,
}

struct CarDealer {
    cars: Vec<Rc<RefCell<Car>>>,
}

struct User {
    car: Option<Rc<RefCell<Car>>>,
}

impl std::default::Default for Car {
    fn default() -> Self {
        Car { model: String::default(), year: 0, price: 0, rent: false }
    }
}

impl Car {
    fn new(model: String, year: u32, price: u32) -> Self {
        Car { model, year, price, rent: false }
    }
}

impl User {
    fn print_car(&self) {
        match &self.car {
            Some(c) => println!("{c:?}"),
            None => println!("No Carrr"),
        }
    }
}

impl CarDealer {
    fn new(cars: Vec<Rc<RefCell<Car>>>) -> Self {
        CarDealer { cars }
    }

    fn add_car(&mut self, c: Car) {
        self.cars.push(Rc::new(RefCell::new(c)));
    }

    fn print_cars(&self) {
        println!("{:?}", self.cars)
    }

    fn rent_user(&mut self, u: &mut User, model: String) {
        let car: Option<&Rc<RefCell<Car>>> = self.cars.iter().find(|c: &&Rc<RefCell<Car>>| c.borrow().model == model);
        
        match car {
            Some(c) => { u.car = Some(c.clone()); c.borrow_mut().rent = true },
            None => { u.car = None; println!("Car not found") },
        }
    }

    // ???
    fn end_rental(&mut self, u: &mut User) {
        match &u.car {
            Some(c) => {
                c.borrow_mut().rent = false;
                u.car = None;
            }
            None => println!("User has no car"),
        }
    }
}

#[test]
fn test_car_dealer() {
    //create cars
    let car1 = Car {
    model: "Audi".to_string(),
    year: 2010,
    price: 10000,
    rent: false,
    };
    let car2 = Car {
    model: "BMW".to_string(),
    year: 2015,
    price: 20000,
    rent: false,
    };
    let car3 = Car {
    model: "Mercedes".to_string(),
    year: 2018,
    price: 30000,
    rent: false,
    };
    let mut car_dealer = CarDealer::new(vec![
    Rc::new(RefCell::new(car1)),
    Rc::new(RefCell::new(car2)),
    Rc::new(RefCell::new(car3)),
    ]);
    let mut user = User { car: None };
    car_dealer.print_cars();
    car_dealer.rent_user(&mut user, "BMW".to_string());
    user.print_car();
    assert_eq!(car_dealer.cars[1].borrow_mut().rent, true);
    car_dealer.print_cars();
    car_dealer.end_rental(&mut user);
    car_dealer.print_cars();
    assert_eq!(car_dealer.cars[0].borrow_mut().rent, false);
}

fn main() {
    println!("Hello, world!");
}
