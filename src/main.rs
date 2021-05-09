use vec3::Vec3;

const IMAGE_WIDTH: u16 = 256;
const IMAGE_HEIGHT: u16 = 256;
type Colour = Vec3;
type Point3 = Vec3;

fn main() {
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_WIDTH);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\nScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let pixel_colour = Colour::new(
                (i as f32 / (IMAGE_WIDTH - 1) as f32) * 256.0,
                (j as f32 / (IMAGE_HEIGHT - 1) as f32) * 256.0,
                0.25 * 256.0,
            );
            pixel_colour.print_u8();
        }
    }
}
