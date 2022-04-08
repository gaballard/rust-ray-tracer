use std::ops;

/**
 * Vector3
 */
#[derive(Debug, Copy, Clone)]
struct Vector3 {
    e: [f32; 3],
}

impl Vector3 {
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
    fn print(&self) {
        println!("{} {} {}", self.e[0], self.e[1], self.e[2]);
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Vector3 {
        Vector3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, v: Vector3) {
        *self = Vector3 {
            e: [self.e[0] + v.e[0], self.e[1] + v.e[1], self.e[2] + v.e[2]],
        }
    }
}

impl ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, t: f32) {
        *self = Vector3 {
            e: [self.e[0] * t, self.e[1] * t, self.e[2] * t],
        }
    }
}

impl ops::DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, t: f32) {
        *self *= 1.0 / t
    }
}

impl ops::Add for Vector3 {
    type Output = Vector3;
    fn add(self, v: Vector3) -> Vector3 {
        Vector3 {
            e: [self.e[0] + v.e[0], self.e[1] + v.e[1], self.e[2] + v.e[2]],
        }
    }
}

impl ops::Add for &Vector3 {
    type Output = Vector3;
    fn add(self, v: &Vector3) -> Vector3 {
        Vector3 {
            e: [self.e[0] + v.e[0], self.e[1] + v.e[1], self.e[2] + v.e[2]],
        }
    }
}

impl ops::Sub for &Vector3 {
    type Output = Vector3;
    fn sub(self, v: &Vector3) -> Vector3 {
        Vector3 {
            e: [self.e[0] - v.e[0], self.e[1] - v.e[1], self.e[2] - v.e[2]],
        }
    }
}

impl ops::Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, v: Vector3) -> Vector3 {
        Vector3 {
            e: [self.e[0] - v.e[0], self.e[1] - v.e[1], self.e[2] - v.e[2]],
        }
    }
}

impl ops::Mul for &Vector3 {
    type Output = Vector3;
    fn mul(self, v: &Vector3) -> Vector3 {
        Vector3 {
            e: [self.e[0] * v.e[0], self.e[1] * v.e[1], self.e[2] * v.e[2]],
        }
    }
}

impl ops::Mul for Vector3 {
    type Output = Vector3;
    fn mul(self, v: Vector3) -> Vector3 {
        Vector3 {
            e: [self.e[0] * v.e[0], self.e[1] * v.e[1], self.e[2] * v.e[2]],
        }
    }
}

impl ops::Mul<f32> for &Vector3 {
    type Output = Vector3;
    fn mul(self, t: f32) -> Vector3 {
        Vector3 {
            e: [self.e[0] * t, self.e[1] * t, self.e[2] * t],
        }
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, t: f32) -> Vector3 {
        Vector3 {
            e: [self.e[0] * t, self.e[1] * t, self.e[2] * t],
        }
    }
}

impl ops::Mul<Vector3> for f32 {
    type Output = Vector3;
    fn mul(self, v: Vector3) -> Vector3 {
        Vector3 {
            e: [self * v.e[0], self * v.e[1], self * v.e[2]],
        }
    }
}

impl ops::Mul<&Vector3> for f32 {
    type Output = Vector3;
    fn mul(self, v: &Vector3) -> Vector3 {
        Vector3 {
            e: [self * v.e[0], self * v.e[1], self * v.e[2]],
        }
    }
}

impl ops::Mul<&Vector3> for &f32 {
    type Output = Vector3;
    fn mul(self, v: &Vector3) -> Vector3 {
        Vector3 {
            e: [self * v.e[0], self * v.e[1], self * v.e[2]],
        }
    }
}

impl ops::Div<f32> for &Vector3 {
    type Output = Vector3;
    fn div(self, t: f32) -> Vector3 {
        self * (1.0 / t)
    }
}

impl ops::Div<f32> for Vector3 {
    type Output = Vector3;
    fn div(self, t: f32) -> Vector3 {
        self * (1.0 / t)
    }
}

/**
 * Vector Utils
 */
fn dot(u: &Vector3, v: &Vector3) -> f32 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}
fn cross(u: &Vector3, v: &Vector3) -> Vector3 {
    Vector3 {
        e: [
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0],
        ],
    }
}
fn unit_vector(v: &Vector3) -> Vector3 {
    v / v.length()
}

/**
 * 3D Point
 */
type Point3 = Vector3;

/**
 * Colors
 */
type Color = Vector3;

fn write_color(pixel_color: Color) {
    println!(
        "{} {} {}",
        (255.999 * pixel_color.x()) as u16,
        (255.999 * pixel_color.y()) as u16,
        (255.999 * pixel_color.z()) as u16
    )
}
fn ray_color(r: Ray) -> Color {
    if hit_sphere(
        &Point3 {
            e: [0.0, 0.0, -1.0],
        },
        0.5,
        &r,
    ) {
        return Color { e: [1.0, 0.0, 0.0] };
    }
    let unit_direction = unit_vector(&r.direction());
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color { e: [1.0, 1.0, 1.0] } + t * Color { e: [0.5, 0.7, 1.0] }
}

/**
 * Rays
 */
struct Ray {
    orig: Point3,
    dir: Vector3,
}

impl Ray {
    fn origin(&self) -> Point3 {
        self.orig
    }
    fn direction(&self) -> Vector3 {
        self.dir
    }

    fn at(&self, t: f32) -> Point3 {
        self.orig + (t * self.dir)
    }
}

fn hit_sphere(center: &Point3, radius: f32, r: &Ray) -> bool {
    let oc = r.origin() - *center;
    let a = dot(&r.direction(), &r.direction());
    let b = 2.0 * dot(&oc, &r.direction());
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c; // quadraticc equationz
    discriminant > 0.0
}

/**
 * Main
 */
fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;

    const IMAGE_WIDTH: u16 = 400;
    const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u16;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3 { e: [0.0, 0.0, 0.0] };
    let horizontal = Vector3 {
        e: [viewport_width, 0.0, 0.0],
    };
    let vertical = Vector3 {
        e: [0.0, viewport_height, 0.0],
    };
    let lower_left_corner = origin
        - horizontal / 2.0
        - vertical / 2.0
        - Vector3 {
            e: [0.0, 0.0, focal_length],
        };

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for y in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", y);
        for x in 0..IMAGE_WIDTH {
            let u = x as f32 / (IMAGE_WIDTH as f32 - 1.0);
            let v = y as f32 / (IMAGE_HEIGHT as f32 - 1.0);
            let r = Ray {
                orig: origin,
                dir: lower_left_corner + u * horizontal + v * vertical - origin,
            };
            let pixel_color: Color = ray_color(r);
            write_color(pixel_color);
        }
    }

    eprintln!("\nDone.");
}

fn tests() {
    let mut a = Vector3 { e: [1.0, 1.0, 1.0] };

    a.print();
    assert_eq!(a.e, [1.0, 1.0, 1.0]);

    a += Vector3 { e: [2.0, 2.0, 2.0] };
    a.print();
    assert_eq!(a.e, [3.0, 3.0, 3.0]);

    a *= 3.0;
    a.print();
    assert_eq!(a.e, [9.0, 9.0, 9.0]);

    a /= 3.0;
    a.print();
    assert_eq!(a.e, [3.0, 3.0, 3.0]);

    let b = a + Vector3 { e: [2.0, 2.0, 2.0] };
    b.print();
    assert_eq!(b.e, [5.0, 5.0, 5.0]);
    let c = b - Vector3 { e: [1.5, 1.5, 1.5] };
    c.print();
    assert_eq!(c.e, [3.5, 3.5, 3.5]);
    let d = c * Vector3 { e: [2.0, 2.0, 2.0] };
    d.print();
    assert_eq!(d.e, [7.0, 7.0, 7.0]);
    let e = d * 0.5;
    e.print();
    assert_eq!(e.e, [3.5, 3.5, 3.5]);
    let f = e / 1.25;
    f.print();
    assert_eq!(f.e, [2.8, 2.8, 2.8]);
    println!("{:?}", unit_vector(&f));
    println!("{:?}", dot(&f, &Vector3 { e: [2.0, 2.0, 2.0] }));
    f.print();
    println!("{:?}", cross(&f, &Vector3 { e: [4.0, 4.0, 4.0] }));
}
