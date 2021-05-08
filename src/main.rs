const IMAGE_WIDTH: u16 = 256;
const IMAGE_HEIGHT: u16 = 256;

fn main() {
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_WIDTH);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\nScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let red = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let green = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let blue = (0.25 * 256.0) as u8;

            let red = (red * 256.0) as u8;
            let green = (green * 256.0) as u8;

            print!("{} {} {}\n", red, green, blue);
        }
    }
}
