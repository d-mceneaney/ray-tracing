mod camera;
mod objects;
mod ray;
mod utils;
mod vec3;
use objects::{HitRecord, Sphere, HittableList};
use ray::Ray;
use utils::{write_color, random_vec3_in_unit_sphere};
use vec3::Vec3;
use rand::Rng;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u16 = 400;
const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u16;
const FOCAL_LENGTH: f32 = 1.0;
const SAMPLES_PER_PIXEL: u8 = 100;
const MAX_DEPTH: u8 = 50;
type Colour = Vec3;
type Point3 = Vec3;

fn ray_colour(ray: &Ray, world: &HittableList, depth: u8) -> Colour {
    if depth <=0 {
        return Colour::new_i32(0, 0, 0);
    }

    let mut record = HitRecord::new();

        if world.hit(ray, 0.0, f32::INFINITY, &mut record) {
            let target: Point3 = record.p + record.normal + random_vec3_in_unit_sphere();
            return ray_colour(&Ray::new(record.p, target - record.p), &world, depth - 1) * 0.5;
           // return (Colour::new_i32(1, 1, 1) + record.normal) * 0.5;
    }
    let unit_direction = ray.direction().unit_vec();
    let t = (unit_direction.y() + 1.0) * 0.5;
    return Colour::new_i32(1, 1, 1) * (1.0 - t) + Colour::new(0.5, 0.7, 1.0) * t;
}

fn main() {
    //world
    let mut world =  HittableList::new();
    world.add(Box::new(Sphere::new( Point3::new(0.0, 0.0, -FOCAL_LENGTH), 0.5)));
    world.add(Box::new(Sphere::new( Point3::new(0.0, -100.5, -FOCAL_LENGTH), 100.0)));

    //Camera
    let camera = camera::Camera::default();

    //render
    let mut rng = rand::thread_rng();
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("Scanlines remaining: {}\n", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_colour: Colour = Colour::new_i32(0, 0, 0);

            for  _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + rng.gen_range(0.0..1.0)) / (IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + rng.gen_range(0.0..1.0)) / (IMAGE_HEIGHT - 1) as f32;
                let ray = camera.get_ray(u, v);
                pixel_colour = pixel_colour + ray_colour(&ray, &world, MAX_DEPTH);
            }
            write_color(pixel_colour, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("Done\n");
}
