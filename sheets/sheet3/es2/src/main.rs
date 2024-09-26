enum _Fuel {
    Diesel,
    Gasoline,
    LPG,
    Methane,
    Electric,
}

enum _IP {
    // V4(u8, u8, u8, u8),
    // V6(u16, u16, u16, u16, u16, u16, u16, u16),
    V4([u8; 4]),
    V6([u16; 8]),
}

struct _Point {
    x: f64,
    y: f64,
    z: f64,
}

fn main() {
    println!("Hello, world!");
}
