use crate::math::*;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: Float,
    u: Vec3,
    v: Vec3
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, up: Vec3, vertical_fov: Float,
            aspect_ratio: Float, aperture: Float, focus_dist: Float) -> Self {
        let theta = vertical_fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;
        let w = Vec3::normalize(look_from - look_at);
        let u = Vec3::normalize(Vec3::cross(up, w));
        let v = Vec3::cross(w, u);

        Camera {
            origin: look_from,
        lower_left_corner: look_from - focus_dist * (half_width * u + half_height * v + w),
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            lens_radius: aperture / 2.0, u: u, v: v
        }
    }

    pub fn get_ray(&self, s: Float, t: Float) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = rd.x() * self.u + rd.y() * self.v;
        Ray::new(self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)
    }
}