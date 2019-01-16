use super::math::*;
use super::material::*;

pub trait Hitable {
    fn hit(&self, ray: Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
}

pub struct HitRecord<'a> {
    pub t: Float,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a (Material + 'a)
}

pub struct Sphere<'a> {
    center: Vec3,
    radius: Float,
    material: &'a Material
}

impl<'a> Sphere<'a> {
    pub fn new(center: Vec3, radius: Float, material: &'a Material) -> Sphere {
        Sphere { center: center, radius: radius, material: material }
    }
}

impl Hitable for Vec<&dyn Hitable> {
fn hit(&self, ray: Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let mut result: Option<HitRecord> = None;
        let mut closest: Float = t_max;
        for hitable in self.iter() {
            if let Some(rec) = hitable.hit(ray, t_min, closest) {
                closest = rec.t;
                result = Some(HitRecord { t: rec.t, p: rec.p, normal: rec.normal, material: rec.material });
            }
        }
        result
    }
}

impl<'a> Hitable for Sphere<'a> {
    fn hit(&self, ray: Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = Vec3::dot(ray.direction(), ray.direction());
        let b = Vec3::dot(oc, ray.direction());
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let sqrt = (b * b - a * c).sqrt();
            let temp1 = (-b - sqrt) / a;
            if temp1 < t_max && temp1 > t_min {
                let p = ray.eval(temp1);
                return Some(HitRecord { t: temp1, p: p, normal: (p - self.center) / self.radius, material: self.material });
            }
            let temp2 = (-b + sqrt) / a;
            if temp2 < t_max && temp2 > t_min {
                let p = ray.eval(temp2);
                return Some(HitRecord { t: temp2, p: p, normal: (p - self.center) / self.radius, material: self.material });
            }
        }
        None
    }
}