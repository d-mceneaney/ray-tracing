use crate::ray::Ray;
use crate::vec3::Vec3;

type Point3 = Vec3;

pub struct Camera {
    origin: Point3,
    aspect_ratio: f32,
    viewport_height: f32,
    focal_length: f32,
    viewport_width: f32,
    lower_left_corner: Point3,
    horizonal_vector: Vec3,
    vertical_vector: Vec3,
}

impl Camera {
    pub fn new(origin: Point3, aspect_ratio: f32, viewport_height: f32, focal_length: f32) -> Self {
        let viewport_width = aspect_ratio * viewport_height;
        let horizonal_vector = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical_vector = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizonal_vector / 2.0 - vertical_vector / 2.0 - Vec3::new(0.0, 0.0, -focal_length);

        Self {
            origin,
            aspect_ratio,
            viewport_height,
            focal_length,
            viewport_width,
            lower_left_corner,
            horizonal_vector,
            vertical_vector,
        }
    }

    pub fn default() -> Self {
        let origin = Point3::new_i32(0, 0, 0);
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let focal_length = 1.0;
        let viewport_width = aspect_ratio * viewport_height;
        let horizonal_vector = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical_vector  = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizonal_vector  / 2.0 - vertical_vector  / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            aspect_ratio,
            viewport_height,
            focal_length,
            viewport_width,
            lower_left_corner,
            horizonal_vector ,
            vertical_vector ,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let ray_endpoint: Point3 = self.lower_left_corner + self.horizonal_vector  * u + self.vertical_vector  * v ;

        Ray::new( self.origin, ray_endpoint - self.origin)
    }
}
