use std::fmt;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;

pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

fn clamp(c: Color) -> Color {
    Color {
        r: c.r.clamp(0.0, 1.0),
        g: c.g.clamp(0.0, 1.0),
        b: c.b.clamp(0.0, 1.0),
    }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        clamp(Self { r: r, g: g, b: b, })
    }

    pub fn get_r(&self) -> f64 {
        self.r
    }

    pub fn get_g(&self) -> f64 {
        self.g
    }

    pub fn get_b(&self) -> f64 {
        self.b
    }

    pub fn set_r(&mut self, r: f64) {
        *self = clamp(Self {
            r: r,
            g: self.g,
            b: self.b,
        })
    }

    pub fn set_g(&mut self, g: f64) {
        *self = clamp(Self {
            r: self.r,
            g: g,
            b: self.b,
        })
    }

    pub fn set_b(&mut self, b: f64) {
        *self = clamp(Self {
            r: self.r,
            g: self.g,
            b: b,
        })
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = (255.0 * self.r) as i32;
        let g = (255.0 * self.g) as i32;
        let b = (255.0 * self.b) as i32;
        write!(f, "{} {} {}", r, g, b)
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        clamp(Self {
                r: self.r + rhs.r,
                g: self.g + rhs.g,
                b: self.b + rhs.b,
        })
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = clamp(Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        });
    }
}

impl Div<f64> for Color {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        clamp(Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        })
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        *self = clamp(Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        });
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        clamp(Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        })
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        clamp(Color {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        })
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        *self = clamp(Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        });
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        clamp(Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        })
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        *self = clamp(Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        });
    }
}