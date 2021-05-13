//!A vector struct with three components of type f32
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {

    ///A new Vec3 can be created using f32 values
    pub fn new(x: f32, y: f32, z:f32) -> Self {
        Self {x, y, z}
    }

    ///A new Vec3 can be created using i32 values
    pub fn new_i32(x: i32, y: i32, z: i32) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            z: z as f32,
        }
    }

    ///The x component of the Vec3
    pub fn x(&self) -> f32 {
        self.x
    }

    ///The y component of the Vec3
    pub fn y(&self) -> f32 {
        self.y
    }

    ///The z component of the Vec3
    pub fn z(&self) -> f32 {
        self.z
    }

    ///Returns the sum of each of the components squared
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    ///Returns the length of the Vec3
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    ///Returns a unit vector of type Vec3
    pub fn unit_vec(&self) -> Self {
            *self / self.length()
    }

    ///Returns the dot product of the Vec3 with another Vec3
    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    ///Returns the cross product of the Vec3 with another Vec3
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    ///Print Vec3 as u8 for use as pixels.
    ///Values larger than the maximum integer value will saturate 
    ///to the maximum value of the integer type.
    pub fn print_u8(&self) {
        println!("{} {} {}", (self.x * 256.0) as u8, (self.y * 256.0) as u8, (self.z * 256.0) as u8)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.x, self.y, self.z)
    }
}

impl PartialOrd for Vec3 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.length() < other.length() {
            Some(Ordering::Less)
        } else if self.length() > other.length() {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, a: f32) -> Self::Output {
        Self {
            x: self.x + a,
            y: self.y + a,
            z: self.z + a,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<f32> for Vec3 {
    type Output = Vec3;

    fn sub(self, s: f32) -> Self::Output {
        Self {
            x: self.x - s,
            y: self.y - s,
            z: self.z - s,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, m: f32) -> Self::Output {
        Self {
            x: self.x * m,
            y: self.y * m,
            z: self.z * m,
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, m: f32) -> Self::Output {
        Self {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, a: f32) {
        self.x = self.x + a;
        self.y = self.y + a;
        self.z = self.z + a;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

impl SubAssign<f32> for Vec3 {
    fn sub_assign(&mut self, a: f32) {
        self.x = self.x - a;
        self.y = self.y - a;
        self.z = self.z - a;
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
        self.z = self.z * rhs.z;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, a: f32) {
        self.x = self.x * a;
        self.y = self.y * a;
        self.z = self.z * a;
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
        self.z = self.z / rhs.z;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, a: f32) {
        self.x = self.x / a;
        self.y = self.y / a;
        self.z = self.z / a;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<u8> for Vec3 {
    type Output = f32;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<u8> for Vec3 {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn vec3_test() {
        let mut a = Vec3::new_i32(1, 1, 2); //length 6.0
        let b = Vec3::new_i32(2, 1, 2); //length 9.0
        let c = Vec3::new_i32(1, 1, 2); //length 6.0
        let d = Vec3::new_i32(-1, -1, -2); //length 6.0

        assert_eq!( a, Vec3 { x: 1.0, y: 1.0, z: 2.0 });
        assert_eq!(a.length_squared(), 6.0);
        assert_eq!(a.length(), 6.0_f32.sqrt());
        assert!(a < b);
        assert!(a != b);
        assert!(b > a);
        assert_eq!(a, c);
        assert_eq!( -a, Vec3 { x: -1.0, y: -1.0, z: -2.0 });
        assert_eq!(-a, d);
        assert_eq!(a.unit_vec(), Vec3 {x: 1.0 / a.length(), y: 1.0 / a.length(), z: 2.0 / a.length()});
        assert_eq!(a.dot(&b), 7.0);
        assert_eq!(a.cross(&b), Vec3 { x: 0.0, y: 2.0, z: -1.0 } );
        assert_eq!(a * 10.0, Vec3 { x: 10.0, y: 10.0, z: 20.0 } );
        assert_eq!(a / 10.0, Vec3 { x: 0.1, y: 0.1, z: 0.2 } );
        assert_eq!(a + 10.0, Vec3 { x: 11.0, y: 11.0, z: 12.0 } );
        assert_eq!(a - 10.0, Vec3 { x: -9.0, y: -9.0, z: -8.0 } );

        a*=10.0;
        assert_eq!(a, Vec3 { x: 10.0, y: 10.0, z: 20.0 } );
        a/=2.0;
        assert_eq!(a, Vec3 { x: 5.0, y: 5.0, z: 10.0 } );
        a+=3.0;
        assert_eq!(a, Vec3 { x: 8.0, y: 8.0, z: 13.0 } );
        a-=1.0;
        assert_eq!(a, Vec3 { x: 7.0, y: 7.0, z: 12.0 } );

        assert_eq!(7.0, a[0]);
        a[0] = 100.0;
        assert_eq!(100.0, a[0]);
    }
}
