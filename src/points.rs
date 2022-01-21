use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn distance(&self, point: &Self) -> f64 {
        let dx = self.x - point.x;
        let dy = self.y - point.y;
        let dz = self.z - point.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&self) -> Self {
        *self / self.length()
    }
    pub fn normalize_mut(&mut self) {
        *self /= self.length();
    }

    pub fn cross(&self, other: Self) -> Self {
        Point3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Add for Point3D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl AddAssign for Point3D {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Sub for Point3D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl SubAssign for Point3D {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl Mul for Point3D {
    type Output = f64;
    // Dot product
    fn mul(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
impl Mul<f64> for Point3D {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Mul<Point3D> for f64 {
    type Output = Point3D;
    fn mul(self, rhs: Point3D) -> Point3D {
        rhs * self
    }
}
impl MulAssign<f64> for Point3D {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Div<f64> for Point3D {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl DivAssign<f64> for Point3D {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl Neg for Point3D {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[test]
fn test_add() {
    let mut first = Point3D::new(1.0, 2.0, 3.0);
    let second = Point3D::new(3.0, 2.0, 1.0);

    let result = Point3D {
        x: 4.0,
        y: 4.0,
        z: 4.0,
    };
    assert_eq!(first + second, result);
    first += second;
    assert_eq!(first, result);
}

#[test]
fn test_sub() {
    let mut first = Point3D::new(1.0, 2.0, 3.0);
    let second = Point3D::new(3.0, 2.0, 1.0);

    let result = Point3D {
        x: -2.0,
        y: 0.0,
        z: 2.0,
    };
    assert_eq!(first - second, result);
    first -= second;
    assert_eq!(first, result);
}

#[test]
fn test_neg() {
    let first = Point3D::new(1.0, 2.0, 3.0);
    let result = Point3D {
        x: -1.0,
        y: -2.0,
        z: -3.0,
    };
    assert_eq!(-first, result);
}

#[test]
fn test_dot() {
    let first = Point3D::new(1.0, 2.0, 3.0);
    let second = Point3D::new(4.0, 5.0, 6.0);
    assert_eq!(first * second, 32.0);
}

#[test]
fn test_mul() {
    let first = Point3D::new(1.0, 2.0, 3.0);
    let result = Point3D {
        x: 3.0,
        y: 6.0,
        z: 9.0,
    };
    assert_eq!(first * 3.0, result);
    assert_eq!(3.0 * first, result);
}

#[test]
fn test_div() {
    let first = Point3D::new(3.0, 6.0, 9.0);
    let result = Point3D {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    assert_eq!(first / 3.0, result);
}

#[test]
fn test_length() {
    let first = Point3D::new(29.0, 22.0, 14.0);
    assert_eq!(first.length(), 39.0);
    let second = Point3D::new(4.0, 5.0, 6.0);
    assert_eq!(second.length_squared(), 77.0)
}

#[test]
fn test_normalize() {
    let mut first = Point3D::new(29.0, 22.0, 14.0);
    let normalized = first.normalize();
    let result = Point3D {
        x: 29.0 / 39.0,
        y: 22.0 / 39.0,
        z: 14.0 / 39.0,
    };
    assert_eq!(normalized, result);
    assert_eq!(normalized.length(), 1.0);

    first.normalize_mut();
    assert_eq!(first, result);
    assert_eq!(first.length(), 1.0);
}

#[test]
fn test_distance() {
    let first = Point3D::new(1.0, 2.0, 3.0);
    let second = Point3D::new(4.0, 5.0, 6.0);
    assert_eq!(first.distance(&second), 27_f64.sqrt());
}
