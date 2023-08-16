use crate::math::*;
use crate::hitable::*;
use crate::texture::*;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray, textures: &[Box<dyn Texture>]) -> bool;
    fn emitted(&self, _u: f32, _v: f32, _p: &Vec3, _textures: &[Box<dyn Texture>]) -> Vec3 {
        return Vec3::zero();
    }
}

#[derive(Copy, Clone)]
pub enum Materials {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight)
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: TextureId
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: TextureId,
    pub roughness: Float
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub refractive_index: Float
}

#[derive(Copy, Clone)]
pub struct DiffuseLight {
    pub emit: TextureId
}


impl Lambertian {
    pub fn new(albedo: TextureId) -> Lambertian {
        Lambertian { albedo: albedo }
    }
}

impl Metal {
    pub fn new(albedo: TextureId, roughness: Float) -> Metal {
        Metal { albedo: albedo, roughness: roughness }
    }
}

impl Dielectric {
    pub fn new(refractive_index: Float) -> Dielectric {
        Dielectric { refractive_index: refractive_index }
    }
}

impl DiffuseLight {
    pub fn new(emit: TextureId) -> DiffuseLight {
        DiffuseLight { emit: emit }
    }
}

impl Material for Materials {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray, textures: &[Box<dyn Texture>]) -> bool {
        match self {
            Materials::Lambertian(lambertian) => lambertian.scatter(ray_in, rec, attenuation, scattered, textures),
            Materials::Metal(metal) => metal.scatter(ray_in, rec, attenuation, scattered, textures),
            Materials::Dielectric(dielectric) => dielectric.scatter(ray_in, rec, attenuation, scattered, textures),
            Materials::DiffuseLight(diffuse_light) => diffuse_light.scatter(ray_in, rec, attenuation, scattered, textures)
        }
    }

    fn emitted(&self, u: f32, v: f32, p: &Vec3, textures: &[Box<dyn Texture>]) -> Vec3 {
        match self {
            Materials::Lambertian(lambertian) => lambertian.emitted(u, v, p, textures),
            Materials::Metal(metal) => metal.emitted(u, v, p, textures),
            Materials::Dielectric(dielectric) => dielectric.emitted(u, v, p, textures),
            Materials::DiffuseLight(diffuse_light) => diffuse_light.emitted(u, v, p, textures)
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray,
            textures: &[Box<dyn Texture>]) -> bool {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target - rec.p);
        *attenuation = textures[self.albedo].value(0.0, 0.0, &rec.p, textures);
        true
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray,
            textures: &[Box<dyn Texture>]) -> bool {
        let reflected = Vec3::reflect(Vec3::normalize(ray_in.direction()), rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.roughness * random_in_unit_sphere());
        *attenuation = textures[self.albedo].value(0.0, 0.0, &rec.p, textures);
        Vec3::dot(scattered.direction(), rec.normal) > 0.0
    }
}

fn schlick(cosine: Float, refractive_index: Float) -> Float {
    let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)

}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray,
            _textures: &[Box<dyn Texture>]) -> bool {
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

impl Material for DiffuseLight {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord, _attenuation: &mut Vec3, _scattered: &mut Ray, _textures: &[Box<dyn Texture>]) -> bool {
        return false;
    }

    fn emitted(&self, u: f32, v: f32, p: &Vec3, textures: &[Box<dyn Texture>]) -> Vec3 {
        return textures[self.emit].value(u, v, p, textures);
    }
}