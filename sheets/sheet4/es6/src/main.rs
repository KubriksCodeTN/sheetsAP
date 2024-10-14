#![allow(dead_code)]

// use std::{path::Display, ptr::DynMetadata};

struct Point {
    x: f32,
    y: f32,
}

struct Circle {
    radius: f32,
    center: Point,
}

struct Rectangle {
    tl: Point,
    br: Point,
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0., y: 0. }
    }
}

impl Default for Circle {
    fn default() -> Self {
        Circle { radius: 1., center: Point::default() }
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Rectangle { tl: Point { x: -1., y: 1. }, br: Point { x: 1., y: -1. } }
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

struct Area {
    a: f32,
}

impl Default for Area {
    fn default() -> Self {
        Area { a: 0. }
    }
}

impl std::fmt::Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The area is: {} cm^2", self.a)
    }
}

trait GetArea {
    fn get_area(&self) -> Area;
}

impl GetArea for Circle {
    fn get_area(&self) -> Area {
        Area { a: self.radius * self.radius * std::f32::consts::PI }
    }
}

impl GetArea for Rectangle {
    fn get_area(&self) -> Area {
        Area { a: ((self.tl.y - self.br.y) * (self.br.x - self.tl.x)).abs() }
    }
}

impl std::ops::Add<Area> for Area {
    type Output = Area;

    fn add(self, rhs: Self) -> Self::Output {
        Area { a: self.a + rhs.a}
    }
}

impl<'a> std::ops::Add<&'a dyn GetArea> for Area {
    type Output = Area;

    fn add(self, rhs: &dyn GetArea) -> Self::Output {
        self + rhs.get_area()
    }
}

fn sum_area(v: &[&dyn GetArea]) -> Area {
    v.iter().fold(Area::default(), |acc: Area, x: &&dyn GetArea| acc + *x)
}

fn main() {
    let r: &dyn GetArea = &Rectangle { tl: Point { x: 3., y: 3. }, br: Point { x: 2., y: 2. } };
    let c: &dyn GetArea = &Circle { radius: 1., center: Point::default() };
    let c1: &dyn GetArea = &Circle { radius: 2., center: Point::default() };
    println!("{}", sum_area(&[r, c, c1]));
}
