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
            Camera::origin(),
            Point3D::new(0.0, 0.0, 1.0),
            Point3D::new(0.0, height, 0.0),
            Point3D::new(width, 0.0, 0.0),
        )
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.position,
            self.lower_left() - self.direction
                + self.horizontal * u
                + self.vertical * v
                + Self::origin(),
        )
    }

    pub fn origin() -> Point3D {
        Point3D::new(0.0, 0.0, 0.0)
    }

    pub fn lower_left(&self) -> Point3D {
        self.position - 0.5 * self.vertical - 0.5 * self.horizontal
    }
}
