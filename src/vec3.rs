use std::cmp::Eq;
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

// impl ops::DivAssign<f32> for Vec3 {
//     fn div_assign(self, t: f32) {
//         self.mul_assign(1.0 / t);
//     }
// }

// impl ops::Neg for Vec3 {
//     fn neg(self) {
//         self.e[0] = -self.e[0];
//         self.e[1] = -self.e[1];
//         self.e[2] = -self.e[2];
//     }
// }

// impl Vec3 {
//     // fn Vec3(&self, e0: f32, e1: f32, e2: f32) {
//     //     self.e = vec![e0, e1, e2];
//     // }

//     fn x(&self) -> f32 {
//         self.e[0]
//     }
//     fn y(&self) -> f32 {
//         self.e[1]
//     }
//     fn z(&self) -> f32 {
//         self.e[2]
//     }

//     /*

//     fn negative(&self) -> &Vec3 {
//         self.e[0] = -self.e[0];
//         self.e[1] = -self.e[1];
//         self.e[2] = -self.e[2];
//         return self;
//     }

//     fn addTo(&self, v: Vec<f32>) -> &Vec3 {
//         self.e[0] += v[0];
//         self.e[1] += v[1];
//         self.e[2] += v[2];
//         return self;
//     }

//     fn multiplyTo(&self, t: f32) -> &Vec3 {
//         self.e[0] *= t;
//         self.e[1] *= t;
//         self.e[2] *= t;
//         return self;
//     }

//     fn divideTo(&self, t: f32) -> &Vec3 {
//         self.multiplyTo(1.0 / t);
//         return self;
//     }

//     fn lengthSquared(&self) -> f32 {
//         self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
//     }

//     fn length(&self) -> f32 {
//         self.lengthSquared().sqrt()
//     }

//     // Utils

//     fn print(&self, v: Vec3) {
//         println!("{} {} {}", v.e[0], v.e[1], v.e[2]);
//     }
//     fn add(&self, u: Vec3, v: Vec3) {
//         self.Vec3(u.e[0] + v.e[0], u.e[1] + v.e[1], u.e[2] + v.e[2])
//     }
//     fn subtract(&self, u: Vec3, v: Vec3) {
//         self.Vec3(u.e[0] - v.e[0], u.e[1] - v.e[1], u.e[2] - v.e[2])
//     }
//     fn multiply(&self, u: Vec3, v: Vec3) {
//         self.Vec3(u.e[0] * v.e[0], u.e[1] * v.e[1], u.e[2] * v.e[2])
//     }
//     fn multiplyBy(&self, t: f32, v: Vec3) {
//         self.Vec3(t * v.e[0], t * v.e[1], t * v.e[2])
//     }
//     fn divide(&self, v: Vec3, t: f32) -> &Vec3 {
//         v.multiplyTo(1.0 / t)
//     }
//     fn dot(&self, u: Vec3, v: Vec3) {
//         u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
//     }
//     fn cross(&self, u: Vec3, v: Vec3) {
//         self.Vec3(
//             u.e[1] * v.e[2] - u.e[2] * v.e[1],
//             u.e[2] * v.e[0] - u.e[0] * v.e[2],
//             u.e[0] * v.e[1] - u.e[1] * v.e[0],
//         );
//     }
//     fn unitVector(&self, v: Vec3) -> &Vec3 {
//         v.divide(v, v.length())
//     }

//     */
// }

// 3D Point
type Point3 = Vec3;

// RGB Color
type Color = Vec3;

fn main() {
    println!("Testing")
}
