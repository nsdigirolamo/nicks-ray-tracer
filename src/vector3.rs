use rand::prelude::*;

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

/// Represents a 3D vector.
#[derive(Clone, Copy)]
pub struct Vector3 {
    /// The vector's x dimension.
    pub x: f64,
    /// The vector's y dimension.
    pub y: f64,
    /// The vector's z dimension.
    pub z: f64,
}

/// Represents a point in 3D space.
pub type Point3 = Vector3;

impl Vector3 {
    
    ///
    /// Returns a Vector3 with the given arguments.
    ///
    /// # Arguments
    /// `x` - The vector's x dimension.
    /// `y` - The vector's y dimension.
    /// `z` - The vector's z dimension.
    ///
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x: x, y: y, z: z, }
    }

    ///
    /// Returns the vector's magnitude squred as an f64.
    ///
    /// # Arguments
    /// * `&self` - The vector.
    ///
    pub fn mag_squared(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    ///
    /// Returns the vector's magnitude as an f64.
    ///
    /// # Arguments
    /// * `&self` - The vector.
    ///
    pub fn mag(&self) -> f64 {
        self.mag_squared().sqrt()
    }

    ///
    /// Returns the vector's unit vector as a Vector3.
    ///
    /// # Arguments
    /// * `&self` - The vector.
    ///
    pub fn unit(&self) -> Self {
        *self / self.mag()
    }

    ///
    /// Returns the cross product between two vectors as a Vector3.
    ///
    /// # Arguments
    /// * `&self` - The vector on the left side of the operation.
    /// * `rhs` - The vector on the right side of the operation.
    ///
    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }

    ///
    /// Returns the dot product between two vectors as a Vector3.
    ///
    /// # Arguments
    /// * `&self` - The vector on the left side of the operation.
    /// * `rhs` - The vector on the right side of the operation.
    ///
    pub fn dot(&self, rhs: Self) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    ///
    /// Returns true if a vector is near zero.
    ///
    /// # Arguments
    /// `&self` - The vector.
    ///
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    /// 
    /// Returns the reflection of a vector off the given normal as a Vector3.
    ///
    /// # Arguments
    /// `&self` - The vector being reflected.
    /// `normal` - The normal the vector is reflected off.
    ///
    pub fn reflect(&self, normal: Vector3) -> Vector3 {
        let unit_normal = normal.unit();
        *self - 2.0 * self.dot(unit_normal) * unit_normal
    }

    ///
    /// Returns the refraction of a vector off the given normal as a Vector3.
    ///
    /// # Arguments
    /// `&self` - The vector being refracted.
    /// `normal` - The normal the vector is refracted off.
    /// `refraction_index` - The refraction index of the material the vector is refracted through.
    ///
    pub fn refract(&self, normal: Vector3, refraction_index: f64) -> Vector3 {
        let cos_theta = (-*self).dot(normal).min(1.0);
        let r_out_perp = refraction_index * (*self + cos_theta * normal);
        let r_out_parallel = -((1.0 - r_out_perp.mag().powf(2.0)).abs().sqrt()) * normal;
        r_out_perp + r_out_parallel
    }
}

///
/// Returns a random Vector3 within the unit sphere.
///
pub fn rand_vector3() -> Vector3 {
    let mut rng = rand::thread_rng();
    
    let v = Vector3::new(
        rng.gen_range(-1.0..1.0),
        rng.gen_range(-1.0..1.0),
        rng.gen_range(-1.0..1.0)
    );

    // Make sure random vector is within the unit sphere.
    if 1.0 <= v.mag() { rand_vector3() } else { v }
}

///
/// Returns a random Vector3 within the unit disk. The z field is always zero.
///
pub fn rand_vector2() -> Vector3 {
    let mut rng = rand::thread_rng();

    let v = Vector3::new(
        rng.gen_range(-1.0..1.0),
        rng.gen_range(-1.0..1.0),
        0.0,
    );

    // Make sure random vector is within the unit disk.
    if 1.0 <= v.mag() { rand_vector2() } else { v }
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
        };
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
        };
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