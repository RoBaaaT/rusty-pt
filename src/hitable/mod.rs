use super::math::*;
use super::material::*;

pub trait Hitable {
    fn hit(&self, ray: Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
}

pub enum Hitables {
    Sphere(Sphere),
    Plane(Plane),
    List(Vec<Hitables>)
}

pub struct HitRecord {
    pub t: Float,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Materials
}

pub struct Sphere {
    center: Vec3,
    radius: Float,
    material: Materials
}

pub struct Plane {
    normal: Vec3,
    distance: Float,
    material: Materials
}

impl Sphere {
    pub fn new(center: Vec3, radius: Float, material: Materials) -> Sphere {
        Sphere { center: center, radius: radius, material: material }
    }
}

impl Plane {
    pub fn new(normal: Vec3, distance: Float, material: Materials) -> Plane {
        Plane { normal: Vec3::normalize(normal), distance: distance, material: material }
    }
}

impl Hitable for Hitables {
    fn hit(&self, ray: Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        match self {
            Hitables::Sphere(sphere) => sphere.hit(ray, t_min, t_max),
            Hitables::Plane(plane) => plane.hit(ray, t_min, t_max),
            Hitables::List(vector) => vector.hit(ray, t_min, t_max)
        }
    }
}

impl Hitable for Vec<Hitables> {
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

impl Hitable for Sphere {
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

impl Hitable for Plane {
    fn hit(&self, ray: Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let denom = Vec3::dot(ray.direction(), self.normal);
        if denom.abs() <= EPSILON {
            None
        } else {
            let plane_to_origin = self.distance * self.normal - ray.origin();
            let t = Vec3::dot(plane_to_origin, self.normal) / denom;
            if t >= t_min && t <= t_max {
                Some(HitRecord {
                    t: t, p: ray.origin() + t * ray.direction(),
                    normal: self.normal,
                    material: self.material })
            } else {
                None
            }
        }
    }
}