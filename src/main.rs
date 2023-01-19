mod vector3;

use crate::vector3::Vector3;
use crate::vector3::Point3;
use crate::vector3::Color3;

fn main() {
    let v = Vector3::new(3.4, 234.23, 9999.9);
    println!("{:?}", v);

    let p = Point3::new(3.4, 234.23, 9999.9);
    println!("{:?}", p);
}
