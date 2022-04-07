use std::ops;

/**
 * Vector3
 */
#[derive(Debug)]
struct Vector3 {
    e: Vec<f32>,
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
    fn dot(&self, v: Vector3) -> f32 {
        self.e[0] * v.e[0] + self.e[1] * v.e[1] + self.e[2] * v.e[2]
    }
    fn cross(&self, v: Vector3) -> Vector3 {
        Vector3 {
            e: vec![
                self.e[1] * v.e[2] - self.e[2] * v.e[1],
                self.e[2] * v.e[0] - self.e[0] * v.e[2],
                self.e[0] * v.e[1] - self.e[1] * v.e[0],
            ],
        }
    }
    fn unit_vector(&self) -> Vector3 {
        self / self.length()
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Vector3 {
        Vector3 {
            e: vec![-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, v: Vector3) {
        *self = Vector3 {
            e: vec![self.e[0] + v.e[0], self.e[1] + v.e[1], self.e[2] + v.e[2]],
        }
    }
}

impl ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, t: f32) {
        *self = Vector3 {
            e: vec![self.e[0] * t, self.e[1] * t, self.e[2] * t],
        }
    }
}

impl ops::DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, t: f32) {
        *self *= 1.0 / t
        // *self = Vector3 {
        //     e: vec![self.e[0] / v.e[0], self.e[1] / v.e[1], self.e[2] / v.e[2]],
        // }
    }
}

// impl ops::Sub for Vector3 {
//     type Output = Vector3;
//     fn sub(self, v: Vector3) -> Vector3 {
//         Vector3 {
//             e: vec![self.e[0] - v.e[0], self.e[1] - v.e[1], self.e[2] - v.e[2]],
//         }
//     }
// }

// impl ops::Mul for Vector3 {
//     type Output = Vector3;
//     fn mul(self, v: Vector3) -> Vector3 {
//         Vector3 {
//             e: vec![self.e[0] * v.e[0], self.e[1] * v.e[1], self.e[2] * v.e[2]],
//         }
//     }
// }

// impl ops::Div<f32> for Vector3 {
//     type Output = Vector3;
//     fn div(self, t: f32) -> Vector3 {
//         Vector3 {
//             e: vec![self.e[0] / t, self.e[1] / t, self.e[2] / t],
//         }
//     }
// }

// trait DotProduct {
//     fn dot(&self, v: Vector3) -> f32;
// }

// trait CrossProduct {
//     fn cross(&self, v: Vector3) -> Vector3;
// }

// impl DotProduct for Vector3 {
//     fn dot(&self, v: Vector3) -> f32 {
//         self.e[0] * v.e[0] + self.e[1] * v.e[1] + self.e[2] * v.e[2]
//     }
// }

// impl CrossProduct for Vector3 {
//     fn cross(&self, v: Vector3) -> Vector3 {
//         Vector3 {
//             e: vec![
//                 self.e[1] * v.e[2] - self.e[2] * v.e[1],
//                 self.e[2] * v.e[0] - self.e[0] * v.e[2],
//                 self.e[0] * v.e[1] - self.e[1] * v.e[0],
//             ],
//         }
//     }
// }

impl ops::Add for Vector3 {
    type Output = Vector3;
    fn add(self, v: Vector3) -> Vector3 {
        Vector3 {
            e: vec![self.e[0] + v.e[0], self.e[1] + v.e[1], self.e[2] + v.e[2]],
        }
    }
}

impl ops::Add for &Vector3 {
    type Output = Vector3;
    fn add(self, v: &Vector3) -> Vector3 {
        Vector3 {
            e: vec![self.e[0] + v.e[0], self.e[1] + v.e[1], self.e[2] + v.e[2]],
        }
    }
}

impl ops::Sub for &Vector3 {
    type Output = Vector3;
    fn sub(self, v: &Vector3) -> Vector3 {
        Vector3 {
            e: vec![self.e[0] - v.e[0], self.e[1] - v.e[1], self.e[2] - v.e[2]],
        }
    }
}

impl ops::Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, v: Vector3) -> Vector3 {
        Vector3 {
            e: vec![self.e[0] - v.e[0], self.e[1] - v.e[1], self.e[2] - v.e[2]],
        }
    }
}

impl ops::Mul for &Vector3 {
    type Output = Vector3;
    fn mul(self, v: &Vector3) -> Vector3 {
        Vector3 {
            e: vec![self.e[0] * v.e[0], self.e[1] * v.e[1], self.e[2] * v.e[2]],
        }
    }
}

impl ops::Mul for Vector3 {
    type Output = Vector3;
    fn mul(self, v: Vector3) -> Vector3 {
        Vector3 {
            e: vec![self.e[0] * v.e[0], self.e[1] * v.e[1], self.e[2] * v.e[2]],
        }
    }
}

impl ops::Mul<f32> for &Vector3 {
    type Output = Vector3;
    fn mul(self, t: f32) -> Vector3 {
        Vector3 {
            e: vec![self.e[0] * t, self.e[1] * t, self.e[2] * t],
        }
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, t: f32) -> Vector3 {
        Vector3 {
            e: vec![self.e[0] * t, self.e[1] * t, self.e[2] * t],
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
 * 3D Point
 */
type Point3 = Vector3;

/**
 * Color
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

/**
 * Rays
 */
struct Ray {
    orig: Point3,
    dir: Vector3,
}

impl Ray {
    // fn at(&self, t: f32) -> Point3 {
    //     self.orig + t * self.dir
    // }
}

/**
 * Main
 */
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

fn tests() {
    let mut a = Vector3 {
        e: vec![1.0, 1.0, 1.0],
    };

    a.print();
    assert_eq!(a.e, vec![1.0, 1.0, 1.0]);

    a += Vector3 {
        e: vec![2.0, 2.0, 2.0],
    };
    a.print();
    assert_eq!(a.e, vec![3.0, 3.0, 3.0]);

    a *= 3.0;
    a.print();
    assert_eq!(a.e, vec![9.0, 9.0, 9.0]);

    a /= 3.0;
    a.print();
    assert_eq!(a.e, vec![3.0, 3.0, 3.0]);

    let b = a + Vector3 {
        e: vec![2.0, 2.0, 2.0],
    };
    b.print();
    assert_eq!(b.e, vec![5.0, 5.0, 5.0]);
    let c = b - Vector3 {
        e: vec![1.5, 1.5, 1.5],
    };
    c.print();
    assert_eq!(c.e, vec![3.5, 3.5, 3.5]);
    let d = c * Vector3 {
        e: vec![2.0, 2.0, 2.0],
    };
    d.print();
    assert_eq!(d.e, vec![7.0, 7.0, 7.0]);
    let e = d * 0.5;
    e.print();
    assert_eq!(e.e, vec![3.5, 3.5, 3.5]);
    let f = e / 1.25;
    f.print();
    assert_eq!(f.e, vec![2.8, 2.8, 2.8]);
    println!("{:?}", f.unit_vector());
    println!(
        "{:?}",
        f.dot(Vector3 {
            e: vec![2.0, 2.0, 2.0]
        })
    );
    f.print();
    println!(
        "{:?}",
        f.cross(Vector3 {
            e: vec![4.0, 4.0, 4.0]
        })
    );
}
