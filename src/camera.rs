/*
// Description: This file provides a camera struct which defines the generation of
//              rays. The camera struct configures the camera's position, orientation, 
//              lens properties, and time range for motion blur. The fields of the 
//              struct contain the lower_left_corner of the view plane, the horizontal
//              dimension vector of the view plane, the vertical dimension vector of 
//              the view plane, the camera's orthonormal basis vectors, the radius of 
//              camera lens, and time range. The get_ray method generates a ray passing
//              through the camera lens.
*/

use crate::{ray::*, utils::*, vec3::*};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
    time0: f32,
    time1: f32,
}
impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
        time0: f32,
        time1: f32,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.).tan();

        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);

        let _focal_length = 1.;

        let origin = lookfrom;
        let horizontal = u * focus_dist * viewport_width;
        let vertical = v * focus_dist * viewport_height;
        let lower_left_corner = origin - horizontal / 2. - vertical / 2. - w * focus_dist;

        let lens_radius = aperture / 2.;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            w,
            u,
            v,
            lens_radius,
            time0,
            time1,
        }
    }
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
            random_double(self.time0, self.time1),
        )
    }
}