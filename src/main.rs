use eliasfl_task_13::*;

fn hit_sphere(center: Point3D, radius: f64, ray: &Ray) -> bool {
    let circ_origin = ray.origin - center;
    let a = ray.direction * ray.direction;
    let b = 2.0 * circ_origin * ray.direction;
    let c = circ_origin * circ_origin - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn ray_color(ray: &Ray) -> [f64; 3] {
    if hit_sphere(Point3D::new(0.0, 0.0, 2.0), 0.5, ray) {
        return [1.0, 0.0, 0.0];
    }
    let t = 0.5 * (ray.direction.normalize().y + 1.0);
    // From white to blue
    [
        (t * 1.0 + t * 0.5),
        (t * 1.0 + t * 0.7),
        (t * 1.0 + t * 1.0),
    ]
}

#[test]
fn test_ray_color() {
    let p = Point3D::new(0.0, 0.0, 0.0);
    let q = Point3D::new(1.0, 0.0, 0.0);
    let r = Ray::new(p, q);
    assert_eq!(ray_color(&r), [0.75, 0.85, 1.0]);
}

#[test]
fn test_ray() {
    let cam = Camera::standard(1.0, 1.0);
    let ray = cam.get_ray(0.5, 1.0);
    println!("{:?}", ray_color(&ray));
}

fn main() {
    let (width, height) = (256, 256);
    let mut img: image::RgbImage = image::ImageBuffer::new(width, height);
    let cam = Camera::standard(1.0, 1.0);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let u = x as f64 / width as f64;
        let v = y as f64 / height as f64;
        let ray = cam.get_ray(u, v);
        *pixel = image::Rgb(ray_color(&ray).map(|v| (v * u8::MAX as f64).round() as u8));
    }

    img.save("output/image.png").unwrap();
}
