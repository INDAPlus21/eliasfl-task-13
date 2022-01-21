use crate::*;

pub struct Ray {
    pub origin: Point3D,
    pub direction: Point3D,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Point3D) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3D {
        self.origin + t * self.direction
    }
}

pub struct HitRecord {
    pub t: f64,
    pub point: Point3D,
    pub normal: Point3D,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[test]
fn test_ray() {
    let first = Point3D::new(1.0, 2.0, 3.0);
    let second = Point3D::new(3.0, 4.0, 5.0);
    let ray = Ray::new(first, second);
    assert_eq!(ray.origin, first);
    assert_eq!(ray.direction, second);
}

#[test]
fn test_ray_at() {
    let first = Point3D::new(1.0, 2.0, 3.0);
    let second = Point3D::new(4.0, 5.0, 6.0);
    let ray = Ray::new(first, second);
    let pos = ray.at(0.5);
    assert_eq!(
        pos,
        Point3D {
            x: 3.0,
            y: 4.5,
            z: 6.0
        }
    )
}
