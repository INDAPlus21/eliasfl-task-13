use crate::*;

pub struct Camera {
    /// Center point of camera position
    pub position: Point3D,
    /// Direction that the camera is pointing
    pub direction: Point3D,
    /// Vertical viewport vector
    pub vertical: Point3D,
    /// Horizontal viewport vector
    pub horizontal: Point3D,
}

impl Camera {
    pub fn new(
        position: Point3D,
        direction: Point3D,
        vertical: Point3D,
        horizontal: Point3D,
    ) -> Self {
        Self {
            position,
            direction,
            vertical,
            horizontal,
        }
    }

    pub fn standard(width: f64, height: f64) -> Self {
        Camera::new(
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(0.0, 0.0, 1.0),
            Point3D::new(0.0, -height, 0.0),
            Point3D::new(width, 0.0, 0.0),
        )
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.position,
            self.lower_left() + self.horizontal * u + self.vertical * v - self.position,
        )
    }

    pub fn lower_left(&self) -> Point3D {
        self.position - self.vertical / 2.0 - self.horizontal / 2.0 - self.direction
    }
}

#[test]
fn test_camera() {
    let cam = Camera::standard(2.0, 2.0);
    let result = Point3D {
        x: -1.0,
        y: 1.0,
        z: -1.0,
    };
    assert_eq!(cam.lower_left(), result);
}
