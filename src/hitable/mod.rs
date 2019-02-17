mod aabb;

use std::sync::Arc;
use std::cmp::Ordering;
use rand::prelude::*;
use super::math::*;
use super::material::*;
pub use crate::hitable::aabb::AABB;

pub trait Hitable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
    fn bounding_box(&self, t0: Float, t1: Float) -> Option<AABB>;
}

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: Float,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Materials
}

#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: Float,
    material: Materials
}

#[derive(Clone)]
pub struct Plane {
    normal: Vec3,
    distance: Float,
    material: Materials
}

#[derive(Clone)]
pub struct Triangle {
    p0: Vec3,
    p1: Vec3,
    p2: Vec3,
    material: Materials
}

#[derive(Clone)]
pub struct BVHNode {
    left: Arc<dyn Hitable>,
    right: Arc<dyn Hitable>,
    bounding_box: AABB
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

impl Triangle {
    pub fn new(p0: Vec3, p1: Vec3, p2:Vec3, material: Materials) -> Triangle {
        Triangle { p0: p0, p1: p1, p2: p2, material: material }
    }
}

impl BVHNode {
    pub fn new(elements: &[Arc<dyn Hitable>], t0: Float, t1: Float) -> BVHNode {
        let dim = rand::thread_rng().gen_range(0, 3);
        let mut els: Vec<Arc<dyn Hitable>> = Vec::from(elements);
        els.sort_unstable_by(|a, b| {
            let a_bbox = a.bounding_box(t0, t1);
            let b_bbox = b.bounding_box(t0, t1);
            if let (Some(abb), Some(bbb)) = (a_bbox, b_bbox) {
                if abb.min[dim] - bbb.min[dim] < 0.0 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            } else {
                panic!("elements of BVH nodes need to have a bounding box.");
            }
        });

        let (left, right): (Arc<dyn Hitable>, Arc<dyn Hitable>) = if els.len() == 1 {
            (els[0].clone(), els[0].clone())
        } else if els.len() == 2 {
            (els[0].clone(), els[1].clone())
        } else {
            (Arc::new(BVHNode::new(&els[0..(els.len() / 2)], t0, t1)), Arc::new(BVHNode::new(&els[(els.len() / 2)..els.len()], t0, t1)))
        };

        let mut min = Vec3::new(MAX_FLOAT, MAX_FLOAT, MAX_FLOAT);
        let mut max = Vec3::new(MIN_FLOAT, MIN_FLOAT, MIN_FLOAT);
        let left_bbox = left.bounding_box(t0, t1);
        let right_bbox = right.bounding_box(t0, t1);
        if let (Some(left), Some(right)) = (left_bbox, right_bbox) {
            for dim in 0..3 {
                min[dim] = if left.min[dim] < min[dim] { left.min[dim] } else { min[dim] };
                max[dim] = if left.max[dim] > max[dim] { left.max[dim] } else { max[dim] };
                min[dim] = if right.min[dim] < min[dim] { right.min[dim] } else { min[dim] };
                max[dim] = if right.max[dim] > max[dim] { right.max[dim] } else { max[dim] };
            }
        } else {
            panic!("elements of BVH nodes need to have a bounding box.");
        }
        BVHNode { left: left, right: right, bounding_box: AABB::new(min, max) }
    }
}

impl Hitable for Vec<Arc<dyn Hitable>> {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
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

    fn bounding_box(&self, t0: Float, t1: Float) -> Option<AABB> {
        let mut min = Vec3::new(MAX_FLOAT, MAX_FLOAT, MAX_FLOAT);
        let mut max = Vec3::new(MIN_FLOAT, MIN_FLOAT, MIN_FLOAT);
        for hitable in self {
            if let Some(bbox) = hitable.bounding_box(t0, t1) {
                for dim in 0..3 {
                    min[dim] = if bbox.min[dim] < min[dim] { bbox.min[dim] } else { min[dim] };
                    max[dim] = if bbox.max[dim] > max[dim] { bbox.max[dim] } else { max[dim] };
                }
            } else {
                return None;
            }
        }
        Some(AABB::new(min, max))
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
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

    fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<AABB> {
        let abs_radius = self.radius.abs();
        let radius_vec = Vec3::new(abs_radius, abs_radius, abs_radius);
        Some(AABB::new(self.center - radius_vec, self.center + radius_vec))
    }
}

impl Hitable for Plane {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
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

    fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<AABB> {
        None
    }
}

impl Hitable for Triangle {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let edge1 = self.p1 - self.p0;
        let edge2 = self.p2 - self.p0;
        let h = Vec3::cross(ray.direction(), edge2);
        let a = Vec3::dot(edge1, h);
        if a.abs() <= EPSILON {
            None
        } else {
            let f = 1.0 / a;
            let s = ray.origin() - self.p0;
            let u = f * Vec3::dot(s, h);
            if u < 0.0 || u > 1.0 {
                None
            } else {
                let q = Vec3::cross(s, edge1);
                let v = f * Vec3::dot(ray.direction(), q);
                if v < 0.0 || u + v > 1.0 {
                    None
                } else {
                    let t = f * Vec3::dot(edge2, q);
                    if t > t_min && t < t_max {
                        Some(HitRecord {
                            t: t, p: ray.origin() + t * ray.direction(),
                            normal: Vec3::normalize(Vec3::cross(edge1, edge2)),
                            material: self.material })
                    } else {
                        None
                    }
                }
            }
        }
    }

    fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<AABB> {
        let mut min = Vec3::new(MAX_FLOAT, MAX_FLOAT, MAX_FLOAT);
        let mut max = Vec3::new(MIN_FLOAT, MIN_FLOAT, MIN_FLOAT);
        for dim in 0..3 {
            min[dim] = if self.p0[dim] < min[dim] { self.p0[dim] } else { min[dim] };
            max[dim] = if self.p0[dim] > max[dim] { self.p0[dim] } else { max[dim] };
            min[dim] = if self.p1[dim] < min[dim] { self.p1[dim] } else { min[dim] };
            max[dim] = if self.p1[dim] > max[dim] { self.p1[dim] } else { max[dim] };
            min[dim] = if self.p2[dim] < min[dim] { self.p2[dim] } else { min[dim] };
            max[dim] = if self.p2[dim] > max[dim] { self.p2[dim] } else { max[dim] };
        }
        Some(AABB::new(min, max))
    }
}

impl Hitable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        if self.bounding_box.hit(&ray, t_min, t_max) {
            let left_rec = self.left.hit(ray, t_min, t_max);
            let right_rec = self.right.hit(ray, t_min, t_max);
            if let (Some(left), Some(right)) = (left_rec, right_rec) {
                if left.t < right.t {
                    Some(HitRecord {
                        t: left.t, p: left.p,
                        normal: left.normal,
                        material: left.material })
                } else {
                    Some(HitRecord {
                        t: right.t, p: right.p,
                        normal: right.normal,
                        material: right.material })
                }
            } else if let Some(left) = left_rec {
                Some(HitRecord {
                    t: left.t, p: left.p,
                    normal: left.normal,
                    material: left.material })
            } else if let Some(right) = right_rec {
                Some(HitRecord {
                    t: right.t, p: right.p,
                    normal: right.normal,
                    material: right.material })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<AABB> {
        Some(self.bounding_box)
    }
}