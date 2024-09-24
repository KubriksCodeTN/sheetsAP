// Write a function e_equals_mc_squared that takes as input a f32 representing the mass, and that
// uses a globally-defined constant containing the value of the speed of light in a vacuum (expressed in
// m/s). The function outputs the energy equivalent to the mass input;

const C : f32 = 299_792_458.;

fn e_equals_mc_squared(m : f32) -> f32{
    return m * C * C;
}

fn main() {
    e_equals_mc_squared(34.);
}
