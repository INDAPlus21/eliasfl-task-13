use eliasfl_task_13::*;

fn hit_sphere(center: Point3D, radius: f64, ray: &Ray) -> f64 {
    let circ_origin = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = circ_origin * ray.direction;
    let c = circ_origin.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(ray: &Ray) -> [f64; 3] {
    let center = Point3D::new(0.0, 0.0, -1.0);
    let hit = hit_sphere(center, 0.5, ray);
    if hit > 0.0 {
        let normal = (ray.at(hit) - center).normalize();
        return [
            0.5 * (normal.x + 1.0),
            0.5 * (normal.y + 1.0),
            0.5 * (normal.z + 1.0),
        ];
    }
    let t = 0.5 * (ray.direction.normalize().y + 1.0);
    [
        ((1.0 - t) * 1.0 + t * 0.5),
        ((1.0 - t) * 1.0 + t * 0.7),
        ((1.0 - t) * 1.0 + t * 1.0),
    ]
}

#[test]
fn test_ray_color() {
    let p = Point3D::new(0.0, 0.0, 0.0);
    let q = Point3D::new(1.0, 0.0, 1.0);
    let r = Ray::new(p, q);
    assert_eq!(ray_color(&r), [0.75, 0.85, 1.0]);
}

#[test]
fn test_ray() {
    let cam = Camera::standard(2.0, 2.0);
    let ray = cam.get_ray(0.5, 1.0);
    println!("{:?}", ray_color(&ray));
}

fn main() {
    let (width, height) = (512, 512);
    let scale = 2.0;
    let mut img: image::RgbImage = image::ImageBuffer::new(width, height);
    let cam = Camera::standard(scale, (height as f64 / width as f64) * scale);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let u = x as f64 / width as f64;
        let v = y as f64 / height as f64;
        let ray = cam.get_ray(u, v);
        // Convert to u8 from float and set pixel
        *pixel = image::Rgb(ray_color(&ray).map(|v| (v * u8::MAX as f64).round() as u8));
    }

    img.save("output/image.png").unwrap();
}
