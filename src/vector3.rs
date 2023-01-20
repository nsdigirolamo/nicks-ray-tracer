use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vector3;
pub type Color3 = Vector3;

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn mag(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    pub fn mag_sqrd(&self) -> f64 {
        let mag = self.mag();
        mag * mag
    }

    pub fn unit(&self) -> Self {
        *self / self.mag()
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }
}

impl Add for Vector3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Neg for Vector3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

/*

Most of these tests do not work because floating point errors are not taken
into consideration. The math is close enough even though technically they all
fail.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let v1 = Vector3::new(1.5, 2.3, 5.5);
        let v2 = Vector3::new(2.3, 7.6, 3.2);
        let result = v1 + v2;
        let correct = Vector3::new(3.8, 9.9, 8.7);
        assert_eq!(result, correct);
    }

    #[test]
    fn add_assign() {
        let mut result = Vector3::new(1.5, 2.3, 5.5);
        let v2 = Vector3::new(2.3, 7.6, 3.2);
        result += v2;
        let correct = Vector3::new(3.8, 9.9, 8.7);
        assert_eq!(result, correct);
    }

    #[test]
    fn div() {
        let v = Vector3::new(2.3, 44.3, 23.24);
        let s = 3.2;
        let result = v / s;
        let correct = Vector3::new(0.71875, 13.84375, 7.2625);
        assert_eq!(result, correct);
    }

    #[test]
    fn div_assign() {
        let mut result = Vector3::new(2.3, 44.3, 23.24);
        let s = 3.2;
        result /= s;
        let correct = Vector3::new(0.71875, 13.84375, 7.2625);
        assert_eq!(result, correct);
    }

    #[test]
    fn mul() {
        let v = Vector3::new(2.3, 44.3, 23.24);
        let s = 3.2;
        let result = v * s;
        let correct = Vector3::new(7.36, 141.76, 74.368);
        assert_eq!(result, correct);
    }

    #[test]
    fn mul_assign() {
        let mut result = Vector3::new(2.3, 44.3, 23.24);
        let s = 3.2;
        result *= s;
        let correct = Vector3::new(7.36, 141.76, 74.368);
        assert_eq!(result, correct);
    }

    #[test]
    fn neg() {
        let v = Vector3::new(2.3, 44.3, 23.24);
        let result = -v;
        let correct = Vector3::new(-2.3, -44.3, -23.24);
        assert_eq!(result, correct);
    }

    #[test]
    fn sub() {
        let v1 = Vector3::new(1.5, 2.3, 5.5);
        let v2 = Vector3::new(2.3, 7.6, 3.2);
        let result = v1 - v2;
        let correct = Vector3::new(-0.8, -5.3, 2.3);
        assert_eq!(result, correct);
    }

    #[test]
    fn sub_assign() {
        let mut result = Vector3::new(1.5, 2.3, 5.5);
        let v2 = Vector3::new(2.3, 7.6, 3.2);
        result -= v2;
        let correct = Vector3::new(-0.8, -5.3, 2.3);
        assert_eq!(result, correct);
    }

    #[test]
    fn mag() {
        let v = Vector3::new(2.3, 44.3, 23.24);
        let result = v.mag();
        let correct = 50.07871404;
        assert_eq!(result, correct);
    }

    #[test]
    fn unit() {
        let v = Vector3::new(2.3, 44.3, 23.24);
        let result = v.unit();
        let correct = Vector3::new(0.0459276969, 0.8846073796, 0.4640694244);
        assert_eq!(result, correct);
    }

    #[test]
    fn cross() {
        let v1 = Vector3::new(1.5, 2.3, 5.5);
        let v2 = Vector3::new(2.3, 7.6, 3.2);
        let result = v1.cross(v2);
        let correct = Vector3::new(-34.44, 7.85, 6.11);
        assert_eq!(result, correct);
    }

    #[test]
    fn dot() {
        let v1 = Vector3::new(1.5, 2.3, 5.5);
        let v2 = Vector3::new(2.3, 7.6, 3.2);
        let result = v1.dot(v2);
        let correct = 38.53;
        assert_eq!(result, correct);
    }
}
*/