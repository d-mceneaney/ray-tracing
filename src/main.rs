use vec3::Vec3;
use ray::Ray;

const IMAGE_WIDTH: u16 = 1920;
const IMAGE_HEIGHT: u16 = 1080;
type Colour = Vec3;
type Point3 = Vec3;

fn main() {
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("Scanlines remaining: {}\n", j);
        for i in 0..IMAGE_WIDTH {
            let pixel_colour = Colour::new(
                (i as f32 / (IMAGE_WIDTH - 1) as f32) * 256.0,
                (j as f32 / (IMAGE_HEIGHT - 1) as f32) * 256.0,
                0.25 * 256.0,
            );
            pixel_colour.print_u8();
        }
    }

    let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
    eprintln!("{}", ray.at(0.1));
}
