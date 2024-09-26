// type Date = (u8 ,u8, u16);
// type Hours = (u8, u8);

use core::fmt;

#[derive(Debug)]
struct Date(u8, u8, u16);
#[derive(Debug)]
struct Hour(u8, u8);

impl fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{:02}/{:02}/{:04}", self.0, self.1, self.2);
    }
}

impl fmt::Display for Hour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{:02}:{:02}", self.0, self.1);
    }
}

#[derive(Debug)]
// #[display(fmt())]ù
struct BoxShipping {
    name: String,
    barcode: String,
    shipment_date: Date,
    shipment_hour: Hour,
}

impl fmt::Display for BoxShipping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}, {}, {}, {}", self.name, self.barcode, self.shipment_date, self.shipment_hour);
    }
}

fn main() {
    let bs: BoxShipping = BoxShipping{ name: "babbo".to_string(), barcode: "babbo".to_string(), 
                                        shipment_date: Date(1, 1, 1), shipment_hour: Hour(12, 30) };
    println!("{bs}");   
}
