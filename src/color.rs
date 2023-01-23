use std::fmt;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;

pub const _WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, };
pub const _BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, };
pub const _GREY: Color = Color { r: 0.5, g: 0.5, b: 0.5, };

pub const _RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, };
pub const _GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, };
pub const _BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, };
pub const _YELLOW: Color = Color { r: 1.0, g: 1.0, b: 0.0, };
pub const _CYAN: Color = Color { r: 0.0, g: 1.0, b: 1.0, };
pub const _PINK: Color = Color { r: 1.0, g: 0.0, b: 1.0, };

pub const _DARK_RED: Color = Color { r: 0.5, g: 0.0, b: 0.0, };
pub const _DARK_GREEN: Color = Color { r: 0.0, g: 0.5, b: 0.0, };
pub const _DARK_BLUE: Color = Color { r: 0.0, g: 0.0, b: 0.5, };
pub const _DARK_YELLOW: Color = Color { r: 0.5, g: 0.5, b: 0.0, };
pub const _DARK_CYAN: Color = Color { r: 0.0, g: 0.5, b: 0.5, };
pub const _DARK_PINK: Color = Color { r: 0.5, g: 0.0, b: 0.5, };

pub const _LIGHT_RED: Color = Color { r: 1.0, g: 0.5, b: 0.5, };
pub const _LIGHT_GREEN: Color = Color { r: 0.5, g: 1.0, b: 0.5, };
pub const _LIGHT_BLUE: Color = Color { r: 0.5, g: 0.5, b: 1.0, };
pub const _LIGHT_YELLOW: Color = Color { r: 1.0, g: 1.0, b: 0.5, };
pub const _LIGHT_CYAN: Color = Color { r: 0.5, g: 1.0, b: 1.0, };
pub const _LIGHT_PINK: Color = Color { r: 1.0, g: 0.5, b: 1.0, };

#[derive(Default, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r: r, g: g, b: b, }
    }

    pub fn gamma_correct(&mut self) {
        *self = Self {
            r: self.r.sqrt(),
            g: self.g.sqrt(),
            b: self.b.sqrt(),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = 255.0 * self.r;
        let g = 255.0 * self.g;
        let b = 255.0 * self.b;
        write!(f, "{} {} {}", r as i32, g as i32, b as i32)
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
                r: self.r + rhs.r,
                g: self.g + rhs.g,
                b: self.b + rhs.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        };
    }
}

impl Div<f64> for Color {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        };
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        Color {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        };
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        };
    }
}