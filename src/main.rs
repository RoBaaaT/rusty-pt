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
use crate::math::*;
use crate::hitable::*;
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

fn color(ray: Ray, world: &Vec<&dyn Hitable>) -> Vec3 {
    let mut rec = HitRecord::default();
    if world.hit(ray, 0.0, MAX_FLOAT, &mut rec) {
        return 0.5 * Vec3::new(rec.normal.x() + 1.0, rec.normal.y() + 1.0, rec.normal.z() + 1.0);
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
    let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let sphere2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    let world: Vec<&dyn Hitable> = vec!(&sphere1, &sphere2);
    let camera = Camera::new();
    for x in 0..width {
        for y in 0..height {
            let mut col = Vec3::zero();
            for _s in 0..samples {
                let ur: Float = rng.gen();
                let vr: Float = rng.gen();
                let u = (x as Float + ur) / width as Float;
                let v = ((height - y) as Float - vr) / height as Float;
                let r = camera.get_ray(u, v);
                col += color(r, &world);
            }
            col /= samples as Float;
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
