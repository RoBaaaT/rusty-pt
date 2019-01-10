use super::math::*;

pub trait Hitable {
    fn hit(&self, ray: Ray, t_min: Float, t_max: Float, rec: &mut HitRecord) -> bool;
}

#[derive(Clone, Default)]
pub struct HitRecord {
    pub t: Float,
    pub p: Vec3,
    pub normal: Vec3
}

pub struct Sphere {
    center: Vec3,
    radius: Float
}

impl Sphere {
    pub fn new(center: Vec3, radius: Float) -> Sphere {
        Sphere { center: center, radius: radius }
    }
}

impl Hitable for Vec<&dyn Hitable> {
fn hit(&self, ray: Ray, t_min: Float, t_max: Float, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::default();
        let mut hit_anything = false;
        let mut closest: Float = t_max;
        for hitable in self.iter() {
            if hitable.hit(ray, t_min, closest, &mut temp_rec) {
                hit_anything = true;
                closest = temp_rec.t;
                rec.t = temp_rec.t;
                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
            }
        }
        hit_anything
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: Ray, t_min: Float, t_max: Float, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = Vec3::dot(ray.direction(), ray.direction());
        let b = Vec3::dot(oc, ray.direction());
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let sqrt = (b * b - a * c).sqrt();
            let temp1 = (-b - sqrt) / a;
            if temp1 < t_max && temp1 > t_min {
                rec.t = temp1;
                rec.p = ray.eval(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
            let temp2 = (-b + sqrt) / a;
            if temp2 < t_max && temp2 > t_min {
                rec.t = temp2;
                rec.p = ray.eval(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
        }
        false
    }
}