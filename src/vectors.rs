use std::ops;

/**
 * Vector3
 */
#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    e: [f32; 3],
}

impl Vector3 {
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        Self { e: [a, b, c] }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }
    pub fn y(&self) -> f32 {
        self.e[1]
    }
    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn length_squared(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn _print(&self) {
        println!("{} {} {}", self.e[0], self.e[1], self.e[2]);
    }
}

/** Negation **/

impl ops::Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Vector3 {
        Vector3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

/** Addition **/

impl ops::Add for Vector3 {
    type Output = Vector3;
    fn add(self, v: Vector3) -> Vector3 {
        Vector3::new(self.e[0] + v.e[0], self.e[1] + v.e[1], self.e[2] + v.e[2])
    }
}

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, v: Vector3) {
        *self = Vector3::new(self.e[0] + v.e[0], self.e[1] + v.e[1], self.e[2] + v.e[2])
    }
}

/** Subtraction **/

impl ops::Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, v: Vector3) -> Vector3 {
        Vector3::new(self.e[0] - v.e[0], self.e[1] - v.e[1], self.e[2] - v.e[2])
    }
}

/** Multiplication **/

impl ops::Mul<f32> for &Vector3 {
    type Output = Vector3;
    fn mul(self, t: f32) -> Vector3 {
        Vector3::new(self.e[0] * t, self.e[1] * t, self.e[2] * t)
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, t: f32) -> Vector3 {
        Vector3::new(self.e[0] * t, self.e[1] * t, self.e[2] * t)
    }
}

impl ops::Mul<Vector3> for f32 {
    type Output = Vector3;
    fn mul(self, v: Vector3) -> Vector3 {
        Vector3::new(self * v.e[0], self * v.e[1], self * v.e[2])
    }
}

impl ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, t: f32) {
        *self = Vector3::new(self.e[0] * t, self.e[1] * t, self.e[2] * t)
    }
}

/** Division **/

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

impl ops::DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, t: f32) {
        *self *= 1.0 / t
    }
}

/**
 * Vector Utils
 */
pub fn dot(u: &Vector3, v: &Vector3) -> f32 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}
pub fn _cross(u: &Vector3, v: &Vector3) -> Vector3 {
    Vector3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}
pub fn unit_vector(v: &Vector3) -> Vector3 {
    v / v.length()
}

/**
 * 3D Point
 */
pub type Point3 = Vector3;

/**
 * Colors
 */
pub type Color = Vector3;

pub fn write_color(pixel_color: &Color) {
    println!(
        "{} {} {}",
        (255.999 * pixel_color.x()) as u16,
        (255.999 * pixel_color.y()) as u16,
        (255.999 * pixel_color.z()) as u16
    )
}
pub fn ray_color(r: &Ray) -> Color {
    let mut t = hit_sphere(Point3::new(0.0, 0.0, -1.0), &0.5, &r);
    if t > 0.0 {
        let normal = unit_vector(&(r.at(t) - Vector3::new(0.0, 0.0, -1.0)));
        return 0.5 * Color::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
    }
    let unit_direction = unit_vector(&r.direction());
    t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

/**
 * Rays
 */
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    orig: Point3,
    dir: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vector3 {
        self.dir
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.orig + (t * self.dir)
    }
}

pub fn hit_sphere(center: Point3, radius: &f32, r: &Ray) -> f32 {
    let oc = r.origin() - center;
    let a = dot(&r.direction(), &r.direction());
    let b = 2.0 * dot(&oc, &r.direction());
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c; // quadraticc equationz
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}
