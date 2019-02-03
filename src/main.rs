use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

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

const TILE_SIZE: u32 = 64;

#[derive(Debug)]
struct RenderTile {
    left: u32,
    width: u32,
    top: u32,
    height: u32
}

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

fn render_thread(width: u32, height: u32, tiles: Arc<Mutex<Vec<RenderTile>>>, samples: usize,
        world: Arc<Hitable>, camera: Arc<Camera>, out: Arc<RwLock<Vec<u8>>>) {
    loop {
        let t = tiles.lock().unwrap().pop();
        if let Some(tile) = t {
            for x in 0..tile.width {
                for y in 0..tile.height {
                    let mut col = Vec3::zero();
                    let global_x = x + tile.left;
                    let global_y = y + tile.top;
                    for _s in 0..samples {
                        let ur: Float = rand::thread_rng().gen();
                        let vr: Float = rand::thread_rng().gen();
                        let u = (global_x as Float + ur) / width as Float;
                        let v = ((height - global_y) as Float - vr) / height as Float;
                        let r = camera.get_ray(u, v);
                        col += color(r, &*world, 0);
                    }
                    col /= samples as Float;
                    col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());
                    let ir = (col.r() * 255.9) as u8;
                    let ig = (col.g() * 255.9) as u8;
                    let ib = (col.b() * 255.9) as u8;
                    let mut data = out.write().unwrap();
                    data[((global_y * width + global_x) * 3 + 0) as usize] = ir;
                    data[((global_y * width + global_x) * 3 + 1) as usize] = ig;
                    data[((global_y * width + global_x) * 3 + 2) as usize] = ib;
                }
            }
        } else {
            return;
        }
    }
}

fn write_output(file: std::fs::File, width: u32, height: u32) -> std::io::Result<()> {
    // output image setup
    let mut encoder = png::Encoder::new(file, width, height);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    let data = Arc::new(RwLock::new(vec![0u8; (width * height * 3) as usize]));

    // render tile setup
    let tiles = Arc::new(Mutex::new(Vec::new()));
    for x in (0..width).step_by(TILE_SIZE as usize) {
        for y in (0..height).step_by(TILE_SIZE as usize) {
            let tile_width = if width - x >= TILE_SIZE { TILE_SIZE } else { width - x };
            let tile_height = if height - y >= TILE_SIZE { TILE_SIZE } else { height - y };
            tiles.lock().unwrap().push(RenderTile { left: x, top: y, width: tile_width, height: tile_height });
        }
    }

    // scene setup
    let samples = 100;
    let mat1 = Materials::Dielectric(Dielectric::new(1.5));
    let mat2 = Materials::Lambertian(Lambertian::new(Vec3::new(0.2, 0.3, 1.0)));
    let mat3 = Materials::Lambertian(Lambertian::new(Vec3::new(0.7, 0.3, 0.2)));
    let mat4 = Materials::Metal(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.8));
    let sphere1 = Hitables::Sphere(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, mat3));
    let sphere2 = Hitables::Sphere(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, mat1));
    let sphere3 = Hitables::Sphere(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, mat4));
    let sphere4 = Hitables::Sphere(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, mat2));
    let sphere5 = Hitables::Sphere(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, mat1));
    let world = Arc::new(Hitables::List(vec!(sphere1, sphere2, sphere3, sphere4, sphere5)));
    let look_from = Vec3::new(-2.0, 2.0, 1.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let camera = Arc::new(Camera::new(look_from, look_at, Vec3::new(0.0, 1.0, 0.0), 40.0,
        width as Float / height as Float,
        0.5, (look_from - look_at).length()));

    // start render threads
    let mut thread_handles = Vec::new();
    for _ in 0..8 {
        let thread_tiles = Arc::clone(&tiles);
        let thread_world = Arc::clone(&world);
        let thread_camera = Arc::clone(&camera);
        let thread_data = Arc::clone(&data);

        let handle = thread::spawn(move || {
            render_thread(width, height, thread_tiles, samples, thread_world, thread_camera, thread_data);
        });
        thread_handles.push(handle);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }

    writer.write_image_data(&data.read().unwrap())?;
    Ok(())
}
