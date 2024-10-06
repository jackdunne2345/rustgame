// Inside view/render.rs
use crate::raycaster::controller::ray::ray_color;
use crate::raycaster::controller::ray::Ray;
use crate::raycaster::controller::sphere::Sphere;
use crate::raycaster::model::vector::Vec3;

pub fn render(width: u32, height: u32) -> Vec<Vec<Vec3>> {
    // Specify return type
    let mut image = vec![vec![Vec3::new(0.0, 0.0, 0.0); width as usize]; height as usize];
    let aspect_ratio = width as f64 / height as f64;
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);

    for j in 0..height {
        for i in 0..width {
            let u = i as f64 / (width as f64 - 1.0);
            let v = j as f64 / (height as f64 - 1.0);

            let direction =
                Vec3::new(u * aspect_ratio * 2.0 - 1.0, v * 2.0 - 1.0, -1.0).normalize();
            let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), direction);
            image[j as usize][i as usize] = ray_color(&ray, &sphere);
        }
    }

    image // Return the image
}
