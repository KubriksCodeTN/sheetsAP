#![allow(dead_code)]

mod point {
    // use std::intrinsics::sqrtf32; // xddd

    pub struct Point {
        pub x: f32,
        pub y: f32,
    }

    impl Point {
        pub fn new(x: f32, y: f32) -> Point {
            return Point{ x, y };
        }

        pub fn distance(&self, p: &Point) -> f32 {
            return ((self.x - p.x) * (self.x - p.x) + (self.y - p.y) * (self.y - p.y)).sqrt();
        }
    }
}

mod line {
    use crate::point::Point;
    pub struct Line {
        s: Point,
        e: Point,
        m: f32,
        q: f32,
    }

    impl Line {
        // y = m * (x - x0) + y0
        pub fn new(s: Point, e: Point) -> Line {
            let m: f32 = (e.y - s.y) / (e.x - s.x);
            let q: f32 = m * -s.x + s.y;
            return Line{ s, e, m, q };
        }

        pub fn contains(&self, p: &Point) -> Result<(), &str> {
            const EPSILON: f32 = 1e-16;
            if (self.m * p.x + self.q - p.y).abs() < EPSILON {
                return Ok(());
            }

            return Err("no");
        }
    }
}

#[cfg(test)]
mod test {
    use crate::point::Point;
    use crate::line::Line;

    #[test]
    pub fn test() {
        let p1: Point = Point::new(1., 1.);
        let p2: Point = Point::new(3., 3.);
        let l: Line = Line::new(p1, p2);
        let p: Point = Point::new(2., 2.);
        let np: Point = Point::new(4., 5.);

        assert_eq!(l.contains(&p), Ok(()));
        assert_eq!(l.contains(&np), Err("no"));        
    }
}

fn main() {
}
