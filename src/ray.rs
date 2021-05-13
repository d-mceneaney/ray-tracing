//!A Ray struct for use in ray-tracing
use crate::vec3::Vec3;

type Point3 = Vec3;

pub struct Ray {
    origin: Point3,
    direction: Vec3
}

impl Ray {
    ///Returns a new Ray containing an origin of type Point3 
    ///and a direction of type Vec3
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self{origin, direction}
    }
    
    ///Returns the Ray origin
    pub fn origin(&self) -> Point3 {self.origin}
 
    ///Returns the Ray direction
    pub fn direction(&self) -> Vec3 {self.direction}

    ///Returns the point on the Ray at value t
    pub fn at(&self, t: f32) -> Point3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::{Ray, Vec3, Point3};
    
    #[test]
    fn ray_test() {
        let ray = Ray::new(Point3::new_i32(0, 0, 0), Vec3::new_i32(1, 1, 1));

        assert_eq!(ray.origin(), Point3::new_i32(0, 0, 0));
        assert_eq!(ray.direction(), Vec3::new_i32(1, 1, 1));
        assert_eq!(ray.at(0.1), Point3::new(0.1, 0.1, 0.1))
    }
}
