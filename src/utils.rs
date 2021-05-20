use crate::vec3::Vec3;
use rand::Rng;

type Colour = Vec3;

pub fn write_color(pixel_colour: Colour, samples_per_pixel: u8) {
    let mut red = pixel_colour.x();
    let mut green = pixel_colour.y();
    let mut blue = pixel_colour.z();

    //divide the colour by the number of samples
    let scale = 1.0 / samples_per_pixel as f32;
    red = (red * scale).sqrt();
    green = (green * scale).sqrt();
    blue = (blue * scale).sqrt();

    println!(
        "{} {} {}",
        (256.0 * red.clamp(0.0, 0.999)) as u8,
        (256.0 * green.clamp(0.0, 0.999)) as u8,
        (256.0 * blue.clamp(0.0, 0.999)) as u8
    )
}

pub fn random_vec3(min: f32, max: f32) -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

pub fn random_vec3_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vec3(-1.0, 1.0);

        if p.length_squared() >= 1.0 {
            continue;
        } else {
            return  p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_vec3_in_unit_sphere().unit_vec()
}

pub fn random_vec3_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_vec3_in_unit_sphere();

    match in_unit_sphere.dot(normal) > 0.0 {
        true => in_unit_sphere,
        false => -in_unit_sphere
    }
}
