use vec3::Vec3;
use ray::Ray;

const ASPECT_RATIO: f32 = 16.0/9.0;
const IMAGE_WIDTH: u16 = 400;
const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f32/ASPECT_RATIO) as u16;
const FOCAL_LENGTH: f32 = -1.0;
type Colour = Vec3;
type Point3 = Vec3;

fn hit_sphere(sphere_center: Point3, sphere_radius: f32, ray: &Ray) -> bool {
   let oc: Vec3 = ray.origin() - sphere_center;
   let a = ray.direction().dot(&ray.direction());
   let b = ray.direction().dot(&oc) * 2.0;
   let c = oc.dot(&oc) - (sphere_radius*sphere_radius);
   let discriminant = b*b - 4.0*a*c;
   discriminant > 0.0
}

fn ray_colour(ray: &Ray) -> Colour {
    let sphere_center = Point3::new(0.0, 0.0, FOCAL_LENGTH);
    let sphere_radius = 0.5;

    match hit_sphere(sphere_center, sphere_radius, ray) {
        true => Colour::new_i32(1, 0, 0),
        false => {
            let unit_direction  = ray.direction().unit_vec(); 
            let t = 0.5 * (unit_direction.y() + 1.0);
            Colour::new_i32(1, 1, 1) * (1.0 - t) + Colour::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn main() {
//Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;
    let origin = Point3::new_i32(0, 0, 0);
    let horizontal_vector = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical_vector = Vec3::new(0.0, viewport_height, 0.0);
    let focal_vector = Vec3::new(0.0, 0.0, focal_length);
    let lower_left_corner: Point3 = origin - horizontal_vector/2.0 - vertical_vector/2.0 - focal_vector;

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("Scanlines remaining: {}\n", j);
        for i in 0..IMAGE_WIDTH {
            let u = horizontal_vector * (i as f32 / ((IMAGE_WIDTH - 1) as f32));
            let v = vertical_vector * (j as f32 / ((IMAGE_HEIGHT - 1) as f32));
            let ray_endpoint: Point3 = lower_left_corner + u + v;
            let ray = Ray::new(origin, ray_endpoint - origin);
            let pixel_colour = ray_colour(&ray);
            pixel_colour.print_u8();
        }
    }

    eprintln!("Done\n");
}
