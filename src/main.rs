struct Vec3 {
    e: vec![0.0, 0.0, 0.0],
}

impl Vec3 {
    fn Vec3(&self, e0: f32, e1: f32, e2: f32) {
        self.e = vec![e0, e1, e2];
    }

    fn x(&self) -> f32 {
        self.e[0]
    }
    fn y(&self) -> f32 {
        self.e[1]
    }
    fn z(&self) -> f32 {
        self.e[2]
    }

    fn negative(&self) -> &Vec3 {
        self.e[0] = -self.e[0];
        self.e[1] = -self.e[1];
        self.e[2] = -self.e[2];
        return self;
    }

    fn addTo(&self, v: Vec<f32>) -> &Vec3 {
        self.e[0] += v[0];
        self.e[1] += v[1];
        self.e[2] += v[2];
        return self;
    }

    fn multiplyTo(&self, t: f32) -> &Vec3 {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
        return self;
    }

    fn divideTo(&self, t: f32) -> &Vec3 {
        self.multiplyTo(1.0 / t);
        return self;
    }

    fn lengthSquared(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    fn length(&self) -> f32 {
        self.lengthSquared().sqrt()
    }

    // Utils

    fn print(&self, v: Vec3) {
        println!("{} {} {}", v.e[0], v.e[1], v.e[2]);
    }
    fn add(&self, u: Vec3, v: Vec3) {
        self.Vec3(u.e[0] + v.e[0], u.e[1] + v.e[1], u.e[2] + v.e[2])
    }
    fn subtract(&self, u: Vec3, v: Vec3) {
        self.Vec3(u.e[0] - v.e[0], u.e[1] - v.e[1], u.e[2] - v.e[2])
    }
    fn multiply(&self, u: Vec3, v: Vec3) {
        self.Vec3(u.e[0] * v.e[0], u.e[1] * v.e[1], u.e[2] * v.e[2])
    }
    fn multiplyBy(&self, t: f32, v: Vec3) {
        self.Vec3(t * v.e[0], t * v.e[1], t * v.e[2])
    }
    fn divide(&self, v: Vec3, t: f32) -> &Vec3 {
        v.multiplyTo(1.0 / t)
    }
    fn dot(&self, u: Vec3, v: Vec3) {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }
    fn cross(&self, u: Vec3, v: Vec3) {
        self.Vec3(
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0],
        );
    }
    fn unitVector(&self, v: Vec3) -> &Vec3 {
        v.divide(v, v.length())
    }
}

// 3D Point
type Point3 = Vec3;

// RGB Color
type Color = Vec3;

fn writeColor(pixelColor: Color) {
    println!(
        "{} {} {}",
        (255.999 * pixelColor.x()) as u16,
        (255.999 * pixelColor.y()) as u16,
        (255.999 * pixelColor.z()) as u16
    )
}

fn main() {
    // Image
    const IMAGE_WIDTH: u16 = 256;
    const IMAGE_HEIGHT: u16 = 256;

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for y in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", y);
        for x in 0..IMAGE_WIDTH {
            let pixelColor: Color = Color {
                e: vec![
                    x as f32 / (IMAGE_WIDTH as f32 - 1.0),
                    y as f32 / (IMAGE_HEIGHT as f32 - 1.0),
                    0.25,
                ],
            };
            writeColor(pixelColor);

            // let r = x as f32 / (IMAGE_WIDTH as f32 - 1.0);
            // let g = y as f32 / (IMAGE_HEIGHT as f32 - 1.0);
            // let b: f32 = 0.25;

            // let ir = (255.999 * r) as u16;
            // let ig = (255.999 * g) as u16;
            // let ib = (255.999 * b) as u16;

            // println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("\nDone.");
}
