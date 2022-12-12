use std::ops;

#[allow(dead_code)]
struct Complex {
    a: f64,
    b: f64,
}

#[allow(dead_code)]
impl Complex {
    fn new (real: f64, imag: f64) -> Complex {
        Complex{a: real, b: imag}
    }

    fn modulus(&self) -> f64 {
        (self.a*self.a + self.b*self.b).sqrt()
    }

    fn argument(&self) -> f64 {
        self.b.atan2(self.a)
    }

    fn to_string(&self) -> String {
        format!("{}{}{}{}", self.a, " + ", self.b, "i")
    }
}

impl ops::Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {a: self.a + other.a, b: self.b + other.b}
    }
}

impl ops::AddAssign for Complex {
    fn add_assign(&mut self, other: Self) {
        *self = Self {a: self.a + other.a, b: self.b + other.b};
    }
}

impl ops::Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {a: self.a - other.a, b: self.b - other.b}
    }
}

impl ops::SubAssign for Complex {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {a: self.a - other.a, b: self.b - other.b};
    }
}

impl ops::Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {a: self.a*other.a - self.b*other.b, b: self.a*other.b + self.b*other.a}
    }
}

impl ops::MulAssign for Complex {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {a: self.a*other.a - self.b*other.b, b: self.a*other.b + self.b*other.a};
    }
}

impl ops::Div for Complex {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let denom = other.a*other.a + other.b * other.b;
        Self {a: (self.a*other.a + self.b*other.b)/(denom), b: (self.b*other.a - self.a*other.b)/denom}
    }
}

impl ops::DivAssign for Complex {
    fn div_assign(&mut self, other: Self) {
        let denom = other.a*other.a + other.b * other.b;
        *self = Self {a: (self.a*other.a + self.b*other.b)/(denom), b: (self.b*other.a - self.a*other.b)/denom};
    }
}

fn main() {
    let mut c = Complex::new(20.0, 0.0);
    c /= Complex::new(5.0, 0.0);

    println!("{}", c.to_string());
}
