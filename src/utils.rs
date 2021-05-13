use crate::vec3::Vec3;

type Colour = Vec3;

pub fn write_color(pixel_colour: Colour, samples_per_pixel: u8) {
    let mut red = pixel_colour.x();
    let mut green = pixel_colour.y();
    let mut blue = pixel_colour.z();

    //divide the colour by the number of samples
    let scale = 1.0 / samples_per_pixel as f32;
    red = red * scale;
    green = green * scale;
    blue = blue * scale;

    println!(
        "{} {} {}",
        (256.0 * red.clamp(0.0, 0.999)) as u8,
        (256.0 * green.clamp(0.0, 0.999)) as u8,
        (256.0 * blue.clamp(0.0, 0.999)) as u8
    )
}
