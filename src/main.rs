use vec3::Vec3;
use ray::Ray;

const ASPECT_RATIO: f32 = 16.0/9.0;
const IMAGE_WIDTH: u16 = 400;
const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f32/ASPECT_RATIO) as u16;
type Colour = Vec3;
type Point3 = Vec3;

fn ray_colour(r: &Ray) -> Colour {
    let unit_direction  = r.direction().unit_vec(); 
    let t = 0.5 * (unit_direction.y() + 1.0);
    Colour::new_i32(1, 1, 1) * (1.0 - t) + Colour::new(0.5, 0.7, 1.0) * t
}

fn main() {
//Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;
    let origin = Point3::new_i32(0, 0, 0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("Scanlines remaining: {}\n", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / ((IMAGE_WIDTH - 1) as f32);
            let v = j as f32 / ((IMAGE_HEIGHT - 1) as f32);
            let r = Ray::new(origin, lower_left_corner + horizontal*u + vertical*v - origin);
            let pixel_colour = ray_colour(&r);
            pixel_colour.print_u8();
        }
    }

    eprintln!("Done\n");
}
