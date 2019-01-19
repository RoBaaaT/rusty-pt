use crate::math::*;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, up: Vec3, vertical_fov: Float, aspect_ratio: Float) -> Self {
        let theta = vertical_fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;
        let w = Vec3::normalize(look_from - look_at);
        let u = Vec3::normalize(Vec3::cross(up, w));
        let v = Vec3::cross(w, u);

        Camera {
            origin: look_from,
            lower_left_corner: look_from - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v
        }
    }

    pub fn get_ray(&self, u: Float, v: Float) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}