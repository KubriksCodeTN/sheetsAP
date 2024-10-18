#![allow(dead_code)]
// use std::marker::PhantomData;

#[derive(Debug, Default)]
struct Open;
#[derive(Debug, Default)]
struct Close;
#[derive(Debug, Default)]
struct Stopped{ _reason: String }

trait Gatable {}

impl Gatable for Open {}
impl Gatable for Close {}
impl Gatable for Stopped {}

#[derive(Debug, Default)]
struct Gate<S: Gatable> {
    _state: S,
}

impl Gate<Open> {
    fn close(self) -> Result<Gate<Close>, Gate<Stopped>> {
        match rand::random::<usize>() % 2 {
            0 => Ok(Gate::default()),
            _ => Err(Gate { _state: Stopped { _reason: "Bah".to_string() } })
        }
    }
} 

impl Gate<Close> {
    fn open(self) -> Result<Gate<Close>, Gate<Stopped>> {
        match rand::random::<usize>() % 2 {
            0 => Ok(Gate::default()),
            _ => Err(Gate { _state: Stopped { _reason: "Boh".to_string() } })
        }
    }
}

impl Gate<Stopped> {
    fn open(self) -> Gate<Open> {
        Gate::default()
    }

    fn close(self) -> Gate<Close> {
        Gate::default()
    }
}

fn main() {
    let g: Gate<Open> = Gate::default();
    println!("{g:?}");
    let gr = g.close();
    println!("{gr:?}");
    if gr.is_err() {
        let g1 = gr.unwrap_err();
        println!("{:?}", g1.close());
    }
}
