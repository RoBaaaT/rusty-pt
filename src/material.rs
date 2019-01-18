use super::math::*;
use super::hitable::*;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    pub albedo: Vec3
}

pub struct Metal {
    pub albedo: Vec3,
    pub roughness: Float
}

pub struct Dielectric {
    pub refractive_index: Float
}


impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo: albedo }
    }
}

impl Metal {
    pub fn new(albedo: Vec3, roughness: Float) -> Metal {
        Metal { albedo: albedo, roughness: roughness }
    }
}

impl Dielectric {
    pub fn new(refractive_index: Float) -> Dielectric {
        Dielectric { refractive_index: refractive_index }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target - rec.p);
        *attenuation = self.albedo;
        true
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(Vec3::normalize(ray_in.direction()), rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.roughness * random_in_unit_sphere());
        *attenuation = self.albedo;
        Vec3::dot(scattered.direction(), rec.normal) > 0.0
    }
}

fn schlick(cosine: Float, refractive_index: Float) -> Float {
    let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)

}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(ray_in.direction(), rec.normal);
        *attenuation = Vec3::one();

        let (outward_normal, ni_over_nt, cosine) = if Vec3::dot(ray_in.direction(), rec.normal) > 0.0 {
            (-rec.normal,
                self.refractive_index,
                self.refractive_index * Vec3::dot(ray_in.direction(), rec.normal) / ray_in.direction().length())
        } else {
            (rec.normal,
                1.0 / self.refractive_index,
                -self.refractive_index * Vec3::dot(ray_in.direction(), rec.normal) / ray_in.direction().length())
        };

       if let Some(refracted) = Vec3::refract(ray_in.direction(), outward_normal, ni_over_nt) {
            if random() < schlick(cosine, self.refractive_index) {
                *scattered = Ray::new(rec.p, reflected);
            } else {
                *scattered = Ray::new(rec.p, refracted);
            }
        } else {
            *scattered = Ray::new(rec.p, reflected);
        }
        true
    }
}