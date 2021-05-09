use vec3::Vec3;

const IMAGE_WIDTH: u16 = 256;
const IMAGE_HEIGHT: u16 = 256;
type Colour = Vec3;
type Point3 = Vec3;

struct Ray {
    origin: Point3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self{origin, direction}
    }

    pub fn origin(&self) -> Point3 {self.origin}
 
    pub fn direction(&self) -> Vec3 {self.direction}

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + self.direction * t
    }
}

fn main() {
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_WIDTH);

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
