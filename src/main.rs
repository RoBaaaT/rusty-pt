use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::Sender;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

extern crate rand;
extern crate progress;
use rand::prelude::*;

extern crate png;
use png::HasParameters;

mod math;
mod hitable;
mod camera;
mod material;
mod texture;
use crate::math::*;
use crate::hitable::*;
use crate::material::*;
use crate::camera::*;
use crate::texture::*;

const TILE_SIZE: u32 = 64;

#[derive(Debug)]
struct RenderTile {
    left: u32,
    width: u32,
    top: u32,
    height: u32
}

fn main() {
    let width = 1920 / 2;
    let height = 1080 / 2;

    let path = Path::new("out/out.png");

    render(path, width, height);
}

fn color(ray: &Ray, world: &Hitable, textures: &[Box<dyn Texture>], depth: u16) -> Vec3 {
    if let Some(rec) = world.hit(ray, 0.001, MAX_FLOAT) {
        let mut scattered: Ray = Ray::new(Vec3::zero(), Vec3::zero());
        let mut attenuation: Vec3 = Vec3::zero();
        if depth < 50 && rec.material.scatter(&ray, &rec, &mut attenuation, &mut scattered, textures) {
            return attenuation * color(&scattered, world, textures, depth + 1);
        } else {
            return Vec3::zero();
        }
    } else {
        let unit_direction = Vec3::normalize(ray.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Vec3::one() + t * Vec3::new(0.5, 0.7, 1.0);
    }
}

fn render_thread(channel: Sender<bool>, width: u32, height: u32, tiles: Arc<Mutex<Vec<RenderTile>>>, samples: usize,
        world: Arc<Hitable>, camera: Arc<Camera>, textures: Arc<Vec<Box<dyn Texture>>>, out: Arc<RwLock<Vec<u8>>>) {
    loop {
        let t = tiles.lock().unwrap().pop();
        let mut local_data = vec![0u8; (TILE_SIZE * TILE_SIZE * 3) as usize];
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
                        col += color(&r, &*world, &*textures, 0);
                    }
                    col /= samples as Float;
                    col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());
                    let ir = (col.r() * 255.9) as u8;
                    let ig = (col.g() * 255.9) as u8;
                    let ib = (col.b() * 255.9) as u8;
                    local_data[((y * TILE_SIZE + x) * 3 + 0) as usize] = ir;
                    local_data[((y * TILE_SIZE + x) * 3 + 1) as usize] = ig;
                    local_data[((y * TILE_SIZE + x) * 3 + 2) as usize] = ib;
                }
            }
            let mut data = out.write().unwrap();
            for y in 0..tile.height {
                let row_offset = (y + tile.top) * width;
                let begin = row_offset + tile.left;
                let end = begin + tile.width;
                let row_range = (begin * 3) as usize..(end * 3) as usize;
                let local_begin = y * TILE_SIZE;
                let local_end = local_begin + tile.width;
                let local_slice = &local_data[(local_begin * 3) as usize..(local_end * 3) as usize];
                let iter = local_slice.iter();
                // TODO: find out if this allocates the _ Vec or is optimized away
                let _: Vec<u8> = data.splice(row_range, iter.cloned()).collect();
            }
            channel.send(true).unwrap();
        } else {
            return;
        }
    }
}

fn render(path: &Path, width: u32, height: u32) {
    let path_display = path.display();
    // output image setup
    let data = Arc::new(RwLock::new(vec![0u8; (width * height * 3) as usize]));

    // render tile setup
    let start_setup = Instant::now();
    let tiles = Arc::new(Mutex::new(Vec::new()));
    for x in (0..width).step_by(TILE_SIZE as usize) {
        for y in (0..height).step_by(TILE_SIZE as usize) {
            let tile_width = if width - x >= TILE_SIZE { TILE_SIZE } else { width - x };
            let tile_height = if height - y >= TILE_SIZE { TILE_SIZE } else { height - y };
            tiles.lock().unwrap().push(RenderTile { left: x, top: y, width: tile_width, height: tile_height });
        }
    }
    let tile_count = tiles.lock().unwrap().len();

    // scene setup
    let samples = 100;
    let gold_texture = Box::new(ConstantTexture::new(Vec3::new(0.8, 0.6, 0.2)));
    let ground_texture = Box::new(CheckerTexture::new(5, 6, 4.0 * PI));
    let wall_texture = Box::new(ConstantTexture::new(Vec3::new(0.6, 0.2, 0.2)));
    let sphere_texture = Box::new(MarbleTexture::new(7.0));
    let white_texture = Box::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0)));
    let ground_even_texture = Box::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 1.0)));
    let ground_odd_texture = Box::new(ConstantTexture::new(Vec3::new(1.0, 0.3, 0.2)));
    let textures: Arc<Vec<Box<dyn Texture>>> = Arc::new(vec!(gold_texture, ground_texture, wall_texture, sphere_texture,
        white_texture, ground_even_texture, ground_odd_texture));
    let mat1 = Materials::Dielectric(Dielectric::new(1.5));
    let mat2 = Materials::Lambertian(Lambertian::new(1));
    let mat3 = Materials::Lambertian(Lambertian::new(3));
    let gold = Materials::Metal(Metal::new(0, 0.8));
    let mat5 = Materials::Lambertian(Lambertian::new(2));
    let mirror = Materials::Metal(Metal::new(4, 0.0));
    let sphere1 = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, mat3));
    let sphere2 = Arc::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.49, mat1));
    let sphere3 = Arc::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, gold));
    let ground = Arc::new(Plane::new(Vec3::new(0.0, 1.0, 0.0), -0.501, mat2));
    let wall = Arc::new(Plane::new(Vec3::new(0.0, 0.0, 1.0), -2.0, mat5));
    let sphere5 = Arc::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, mat1));
    let tri1 = Arc::new(Triangle::new(Vec3::new(2.0, 0.0, -2.0),
        Vec3::new(2.0, 1.5, -1.5), Vec3::new(-2.0, 0.0, -2.0), mirror));
    let tri2 = Arc::new(Triangle::new(Vec3::new(2.0, 1.5, -1.5),
        Vec3::new(-2.0, 1.5, -1.5), Vec3::new(-2.0, 0.0, -2.0), mirror));
    let bvh_elements: Vec<Arc<dyn Hitable>> = vec!(sphere1, sphere2, sphere3, sphere5, tri1, tri2);
    let bvh = Arc::new(BVHNode::new(&bvh_elements, 0.0, 0.0));
    let world: Arc<Vec<Arc<dyn Hitable>>> = Arc::new(vec!(ground, wall, bvh));
    let look_from = Vec3::new(-3.0, 1.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let camera = Arc::new(Camera::new(look_from, look_at, Vec3::new(0.0, 1.0, 0.0), 40.0,
        width as Float / height as Float,
        0.0, (look_from - look_at).length()));
    let elapsed_setup = start_setup.elapsed();

    // start render threads
    let start_render = Instant::now();
    let mut thread_handles = Vec::new();
    let (tx, rx) = mpsc::channel();
    for _ in 0..8 {
        let thread_tiles = Arc::clone(&tiles);
        let thread_world = Arc::clone(&world);
        let thread_camera = Arc::clone(&camera);
        let thread_textures = Arc::clone(&textures);
        let thread_data = Arc::clone(&data);
        let thread_tx = tx.clone();

        let handle = thread::spawn(move || {
            render_thread(thread_tx, width, height, thread_tiles, samples, thread_world, thread_camera, thread_textures,
                thread_data);
        });
        thread_handles.push(handle);
    }

    let mut progress_bar = progress::Bar::new();
    progress_bar.set_job_title("Rendering");

    let mut rendered_tiles = 0;
    while rendered_tiles < tile_count {
        rx.recv().unwrap();
        rendered_tiles += 1;
        while !rx.try_recv().is_err() {
            rendered_tiles += 1;
        }
        progress_bar.set_job_title(&format!("Rendering ({}/{} tiles complete)", rendered_tiles + 1, tile_count));
        progress_bar.reach_percent((((rendered_tiles + 1) as f32 / tile_count as f32) * 100.0) as i32);

        let file = match File::create(path) {
            Err(why) => panic!("couldn't create {}: {}", path_display, why.description()),
            Ok(file) => file
        };
        let mut encoder = png::Encoder::new(file, width, height);
        encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
        let mut writer = match encoder.write_header() {
            Err(why) => panic!("couldn't write png header to {}: {}", path_display, why.description()),
            Ok(writer) => writer
        };
        match writer.write_image_data(&data.read().unwrap()) {
            Err(why) => panic!("couldn't write image data to {}: {}", path_display, why.description()),
            Ok(_) => ()
        };
    }
    let elapsed_render = start_render.elapsed();

    println!("setup: \t{}.{:09} s", elapsed_setup.as_secs(), elapsed_setup.subsec_nanos() / 1000000);
    println!("render:\t{}.{:03} s", elapsed_render.as_secs(), elapsed_render.subsec_nanos() / 1000000);

    for handle in thread_handles {
        handle.join().unwrap();
    }
}
