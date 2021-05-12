mod ray;
mod objects;
use vec3::Vec3;
use ray::Ray;
use objects::{Sphere, HitRecord, Hittable};
 
const ASPECT_RATIO: f32 = 16.0/9.0;
const IMAGE_WIDTH: u16 = 400;
const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f32/ASPECT_RATIO) as u16;
const FOCAL_LENGTH: f32 = 1.0;
type Colour = Vec3;
type Point3 = Vec3;

fn ray_colour(ray: &Ray, world: &mut Vec<Box<dyn Hittable>>) -> Colour {
    let mut record = HitRecord::new();

    for object in world {
        if object.hit(ray, 0.0, f32::INFINITY, &mut record)
        {
            return ((Colour::new_i32(1, 1, 1) + record.normal)) * 0.5;
        }
    }
    let unit_direction = ray.direction().unit_vec();
    let t = (unit_direction.y() + 1.0) * 0.5;
    
    return Colour::new_i32(1,1,1) * (1.0-t) + Colour::new(0.5, 0.7, 1.0) * t;
}

fn main() {
    //world
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -FOCAL_LENGTH), 0.5))); 
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -FOCAL_LENGTH), 100.0))); 
    
    //Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let origin = Point3::new_i32(0, 0, 0);
    let horizontal_vector = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical_vector = Vec3::new(0.0, viewport_height, 0.0);
    let focal_vector = Vec3::new(0.0, 0.0, FOCAL_LENGTH);
    let lower_left_corner: Point3 = origin - horizontal_vector/2.0 - vertical_vector/2.0 - focal_vector;

    //render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("Scanlines remaining: {}\n", j);
        for i in 0..IMAGE_WIDTH {
            let u = horizontal_vector * (i as f32 / ((IMAGE_WIDTH - 1) as f32));
            let v = vertical_vector * (j as f32 / ((IMAGE_HEIGHT - 1) as f32));
            let ray_endpoint: Point3 = lower_left_corner + u + v;
            let ray = Ray::new(origin, ray_endpoint - origin);
            let pixel_colour = ray_colour(&ray, &mut world);
            pixel_colour.print_u8();
        }
    }

    eprintln!("Done\n");
}
