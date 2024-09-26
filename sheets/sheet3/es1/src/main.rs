fn is_it_luhn(s: String) -> bool {
    if s.len() < 2 {
        return false;
    }

    let s: String = s.replace(" ", "");

    if !s.chars().all(|x| return x.is_digit(10)) {
        return false;
    }

    let sum: u32 = s.chars().rev().enumerate().fold(0, |acc: u32, (i, c)| {
        if (i + 1) % 2 == 0 {
            let mut tmp: u32 = c.to_digit(10).unwrap() * 2;
            if tmp > 9 { tmp -= 9; }
            return acc + tmp;
        }
        
        return acc + c.to_digit(10).unwrap();
    });

    return sum % 10 == 0;
}

fn main() {
    println!("{}", is_it_luhn("s".to_string()));
    println!("{}", is_it_luhn("12".to_string()));
    println!("{}", is_it_luhn("4539319503436467".to_string()));
    println!("{}", is_it_luhn("4539 3195 0343 6467".to_string()));
    println!("{}", is_it_luhn("4539319503436r67".to_string()));
    println!("{}", is_it_luhn("453931950!!436467".to_string()));
    println!("{}", is_it_luhn("378282246310005".to_string()));
    println!("{}", is_it_luhn("5019717010103742".to_string()));
    println!("{}", is_it_luhn("501971701010374222".to_string()));
}
