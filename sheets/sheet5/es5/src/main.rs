struct Pair(i32, String);

impl std::ops::Add<i32> for Pair {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Pair(self.0 + rhs, self.1)
    }
}

impl std::ops::Sub<i32> for Pair {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        Self(self.0 - rhs, self.1)
    }
}

impl std::ops::Add<&str> for Pair {
    type Output = Self;

    fn add(self, rhs: &str) -> Self::Output {
        Self(self.0, self.1 + rhs)
    }
}

impl std::ops::Sub<&str> for Pair {
    type Output = Self;

    fn sub(self, rhs: &str) -> Self::Output {
        Self(self.0, self.1.replace(rhs, ""))
    }
}

impl std::ops::Add for Pair {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + &rhs.1)
    }
}

impl std::ops::Sub for Pair {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1.replace(&rhs.1, ""))
    }
}

// this shuold be u32 but the text asks for i32?
impl std::ops::Mul<i32> for Pair {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        // if rhs < 0 { panic!() }

        Pair(self.0.saturating_pow(rhs as u32), self.1.repeat(rhs as usize))
    }
}

fn main() {
    println!("Hello, world!");
}
