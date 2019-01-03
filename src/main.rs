use std::error::Error;
use std::path::Path;
use std::fs::File;

extern crate png;
use png::HasParameters;

mod math;

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

fn color(ray: math::Ray) -> math::Vec3 {
    let unit_direction = math::Vec3::normalize(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * math::Vec3::one() + t * math::Vec3::new(0.5, 0.7, 1.0)
}

fn write_output(file: std::fs::File, width: u32, height: u32) -> std::io::Result<()> {
    let mut encoder = png::Encoder::new(file, width, height);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    let mut data = vec![0u8; (width * height * 3) as usize];

    // trace rays for each pixel
    let lower_left_corner = math::Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = math::Vec3::new(4.0, 0.0, 0.0);
    let vertical = math::Vec3::new(0.0, 2.0, 0.0);
    let origin = math::Vec3::new(0.0, 0.0, 0.0);
    for x in 0..width {
        for y in 0..height {
            let u = x as math::Float / width as math::Float;
            let v = (height - y) as math::Float / height as math::Float;
            let r = math::Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(r);
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
