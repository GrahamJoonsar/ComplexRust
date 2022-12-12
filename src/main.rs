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

impl ops::Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {a: self.a - other.a, b: self.b - other.b}
    }
}

impl ops::Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {a: self.a*other.a - self.b*other.b, b: self.a*other.b + self.b*other.a}
    }
}

impl ops::Div for Complex {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let denom = other.a*other.a + other.b * other.b;
        Self {a: (self.a*other.a + self.b*other.b)/(denom), b: (self.b*other.a - self.a*other.b)/denom}
    }
}

fn main() {
    println!("{}", (Complex::new(1.0, 1.0) / Complex::new(0.0, 1.0)).to_string())

    
}
