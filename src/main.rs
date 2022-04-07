use std::ops;

#[derive(Debug)]
struct Vec3 {
    e: Vec<f32>,
}

impl Vec3 {
    fn x(&self) -> f32 {
        self.e[0]
    }
    fn y(&self) -> f32 {
        self.e[1]
    }
    fn z(&self) -> f32 {
        self.e[2]
    }

    fn length_squared(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            e: vec![-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        *self = Vec3 {
            e: vec![self.e[0] + v.e[0], self.e[1] + v.e[1], self.e[2] + v.e[2]],
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, v: Vec3) {
        *self = Vec3 {
            e: vec![self.e[0] - v.e[0], self.e[1] - v.e[1], self.e[2] - v.e[2]],
        }
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, v: Vec3) {
        *self = Vec3 {
            e: vec![self.e[0] * v.e[0], self.e[1] * v.e[1], self.e[2] * v.e[2]],
        }
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, v: Vec3) {
        *self = Vec3 {
            e: vec![self.e[0] / v.e[0], self.e[1] / v.e[1], self.e[2] / v.e[2]],
        }
    }
}

trait DotProduct {
    fn dot(&self, v: Vec3) -> f32;
}

trait CrossProduct {
    fn cross(&self, v: Vec3) -> Vec3;
}

impl DotProduct for Vec3 {
    fn dot(&self, v: Vec3) -> f32 {
        self.e[0] * v.e[0] + self.e[1] * v.e[1] + self.e[2] * v.e[2]
    }
}

impl CrossProduct for Vec3 {
    fn cross(&self, v: Vec3) -> Vec3 {
        Vec3 {
            e: vec![
                self.e[1] * v.e[2] - self.e[2] * v.e[1],
                self.e[2] * v.e[0] - self.e[0] * v.e[2],
                self.e[0] * v.e[1] - self.e[1] * v.e[0],
            ],
        }
    }
}

fn print(v: &Vec3) {
    println!("{} {} {}", v.e[0], v.e[1], v.e[2]);
}
fn add(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        e: vec![u.e[0] + v.e[0], u.e[1] + v.e[1], u.e[2] + v.e[2]],
    }
}
fn subtract(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        e: vec![u.e[0] - v.e[0], u.e[1] - v.e[1], u.e[2] - v.e[2]],
    }
}
fn multiply(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        e: vec![u.e[0] * v.e[0], u.e[1] * v.e[1], u.e[2] * v.e[2]],
    }
}
fn divide(v: &Vec3, t: f32) -> Vec3 {
    Vec3 {
        e: vec![v.e[0] / t, v.e[1] / t, v.e[2] / t],
    }
}
fn dot(u: &Vec3, v: &Vec3) -> f32 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}
fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        e: vec![
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0],
        ],
    }
}
fn unit_vector(v: &Vec3) -> Vec3 {
    divide(v, v.length())
}

// 3D Point
type Point3 = Vec3;

// RGB Color
type Color = Vec3;

fn write_color(pixel_color: Color) {
    println!(
        "{} {} {}",
        (255.999 * pixel_color.x()) as u16,
        (255.999 * pixel_color.y()) as u16,
        (255.999 * pixel_color.z()) as u16
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
            let pixel_color: Color = Color {
                e: vec![
                    x as f32 / (IMAGE_WIDTH as f32 - 1.0),
                    y as f32 / (IMAGE_HEIGHT as f32 - 1.0),
                    0.25,
                ],
            };
            write_color(pixel_color);
        }
    }

    eprintln!("\nDone.");
}
