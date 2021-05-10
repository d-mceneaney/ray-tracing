use vec3::Vec3;
use ray::Ray;

const ASPECT_RATIO: f32 = 16.0/9.0;
const IMAGE_WIDTH: u16 = 400;
const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f32/ASPECT_RATIO) as u16;
const SPHERE_RADIUS: f32 = 0.5;
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
    let horizontal_vector = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical_vector = Vec3::new(0.0, viewport_height, 0.0);
    let focal_vector = Vec3::new(0.0, 0.0, focal_length);
    let lower_left_corner: Point3 = origin - horizontal_vector/2.0 - vertical_vector/2.0 - focal_vector;
    let sphere_center: Point3 = origin - Vec3::new(0.0, 0.0, focal_length);

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("Scanlines remaining: {}\n", j);
        for i in 0..IMAGE_WIDTH {
            let u = horizontal_vector * (i as f32 / ((IMAGE_WIDTH - 1) as f32));
            let v = vertical_vector * (j as f32 / ((IMAGE_HEIGHT - 1) as f32));
            let ray_endpoint: Point3 = lower_left_corner + u + v;

            //if the distance between the ray endpoint and the sphere center is less than 
            //the sphere radius then the ray endpoint must be inside the sphere,
            //in which case, colour it red
            if (sphere_center - ray_endpoint).length() <= SPHERE_RADIUS {
                let pixel_colour = Colour::new_i32(1,0,0);
                pixel_colour.print_u8();
            }
            else {
                let r = Ray::new(origin, ray_endpoint - origin);
                let pixel_colour = ray_colour(&r);
                pixel_colour.print_u8();
            }
            //pixel_colour.print_u8();
        }
    }

    eprintln!("Done\n");
}
