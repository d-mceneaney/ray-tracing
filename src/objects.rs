use crate::ray::Ray;
use crate::vec3::Vec3;

type Point3 = Vec3;

pub struct HitRecord {
    p: Point3,
    pub normal: Vec3,
    t: f32,
    front_face: bool
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3::new_i32(0,0,0),
            normal: Vec3::new_i32(0, 0, 0),
            t: 0.0,
            front_face: false
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
       self.front_face = ray.direction().dot(&outward_normal) < 0.0; 
        self.normal = match self.front_face {
            true => outward_normal,
            false => -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&mut self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool;
}

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&mut self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let h = oc.dot(&ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        } else {//if 1st root is not within range, try the next root
            let sqrtd = discriminant.sqrt();
            let mut root = (-h - sqrtd) / a;

            if root < t_min || root > t_max {
                root = (-h + sqrtd) / a;

                if root < t_min || root > t_max {
                    return false;
                }
            }
            record.t = root;
            record.p = ray.at(record.t);
            let outward_normal = (record.p - self.center) / self.radius;
            record.set_face_normal(ray, outward_normal);
            true
        }
    }
}
