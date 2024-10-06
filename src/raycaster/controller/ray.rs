use crate::raycaster::controller::sphere::Sphere;
use crate::raycaster::model::vector::Vec3;
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn point_at(&self, t: f64) -> Vec3 {
        self.origin.add(&self.direction.mul(t))
    }
}

pub fn ray_color(ray: &Ray, sphere: &Sphere) -> Vec3 {
    if let Some(t) = sphere.hit(ray) {
        let hit_point = ray.point_at(t);
        let normal = hit_point.sub(&sphere.center).normalize();
        return Vec3::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0).mul(0.5);
    }

    // Background gradient
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0)
        .mul(1.0 - t)
        .add(&Vec3::new(0.5, 0.7, 1.0).mul(t))
}
