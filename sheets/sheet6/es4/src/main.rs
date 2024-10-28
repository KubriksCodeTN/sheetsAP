#![allow(dead_code, non_snake_case)]

#[derive(Debug)]
struct PublicStreetLight {
    id: u32,
    on: bool,
    burn_out: bool,
}

struct PublicIllumination {
    lights: Vec<PublicStreetLight>,
}

impl std::default::Default for PublicIllumination {
    fn default() -> Self {
        PublicIllumination { lights: vec![] }
    }
}

impl std::default::Default for PublicStreetLight {
    fn default() -> Self {
        PublicStreetLight { id: 0, on: false, burn_out: false }
    }
}

impl PublicIllumination {
    fn new(lights: Vec<PublicStreetLight>) -> Self {
        PublicIllumination { lights }
    }
}

impl PublicStreetLight {
    fn new(id: u32, on: bool, burn_out: bool) -> Self {
        PublicStreetLight { id, on, burn_out }
    }
}

impl std::iter::Iterator for PublicIllumination {
    type Item = PublicStreetLight;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(i) = self.lights.iter().position(|x: &PublicStreetLight| x.burn_out) {
            return Some(self.lights.remove(i));
        }

        None
    }
}

#[test]
fn test_1() {
//create new streetlights
    let streetlight = PublicStreetLight::new(1, true, true);
    let streetlight2 = PublicStreetLight::new(2, true, false);
    let streetlight3 = PublicStreetLight::new(3, true, false);
    let streetlight4 = PublicStreetLight::new(4, false, true);
    let publicIllumination =
    PublicIllumination::new(vec![streetlight, streetlight2, streetlight3,
    streetlight4]);
    for a in publicIllumination {
        println!("{:?}", a);
    }
}

fn main() {
    println!("Hello, world!");
}
