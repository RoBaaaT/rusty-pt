use std::error::Error;
use std::path::Path;
use std::fs::File;

extern crate rand;
use rand::prelude::*;

extern crate png;
use png::HasParameters;

mod math;
mod hitable;
mod camera;
mod material;
use crate::math::*;
use crate::hitable::*;
use crate::material::*;
use crate::camera::*;

fn main() {
    let width = 300;
    let height = 150;

    let path = Path::new("out/out.png");
    let path_display = path.display();
    let file = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", path_display, why.description()),
        Ok(file) => file
    };

    match write_output(file, width, height) {
        Err(why) => panic!("couldn't write to {}: {}", path_display, why.description()),
        Ok(_) => println!("wrote output to {}", path_display)
    }
}

fn color(ray: Ray, world: &Hitable, depth: u16) -> Vec3 {
    if let Some(rec) = world.hit(ray, 0.001, MAX_FLOAT) {
        let mut scattered: Ray = Ray::new(Vec3::zero(), Vec3::zero());
        let mut attenuation: Vec3 = Vec3::zero();
        if depth < 50 && rec.material.scatter(&ray, &rec, &mut attenuation, &mut scattered) {
            return attenuation * color(scattered, world, depth + 1);
        } else {
            return Vec3::zero();
        }
    } else {
        let unit_direction = Vec3::normalize(ray.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Vec3::one() + t * Vec3::new(0.5, 0.7, 1.0);
    }
}

fn write_output(file: std::fs::File, width: u32, height: u32) -> std::io::Result<()> {
    let mut encoder = png::Encoder::new(file, width, height);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    let mut data = vec![0u8; (width * height * 3) as usize];
    let mut rng = rand::thread_rng();

    // trace rays for each pixel
    let samples = 10;
    let mat1 = Dielectric::new(1.5);
    let mat2 = Lambertian::new(Vec3::new(0.2, 0.3, 1.0));
    let mat3 = Lambertian::new(Vec3::new(0.7, 0.3, 0.2));
    let mat4 = Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.8);
    let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, &mat3);
    let sphere2 = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, &mat1);
    let sphere3 = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, &mat4);
    let sphere4 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, &mat2);
    let sphere5 = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, &mat1);
    let world: Vec<&dyn Hitable> = vec!(&sphere1, &sphere2, &sphere3, &sphere4, &sphere5);
    let look_from = Vec3::new(-2.0, 2.0, 1.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let camera = Camera::new(look_from, look_at, Vec3::new(0.0, 1.0, 0.0), 40.0, width as Float / height as Float,
        0.5, (look_from - look_at).length());
    for x in 0..width {
        for y in 0..height {
            let mut col = Vec3::zero();
            for _s in 0..samples {
                let ur: Float = rng.gen();
                let vr: Float = rng.gen();
                let u = (x as Float + ur) / width as Float;
                let v = ((height - y) as Float - vr) / height as Float;
                let r = camera.get_ray(u, v);
                col += color(r, &world, 0);
            }
            col /= samples as Float;
            col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());
            let ir = (col.r() * 255.9) as u8;
            let ig = (col.g() * 255.9) as u8;
            let ib = (col.b() * 255.9) as u8;
            data[((y * width + x) * 3 + 0) as usize] = ir;
            data[((y * width + x) * 3 + 1) as usize] = ig;
            data[((y * width + x) * 3 + 2) as usize] = ib;
        }
    }

    writer.write_image_data(&data)?;
    Ok(())
}
