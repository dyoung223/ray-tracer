/*
// Description: This file defines a ray struct and ray_color function. The ray struct 
//              represents a ray 3-d space with an origin, direction and time component.
//              There are methods for creating a new ray, accessing fields and computing
//              a point along the ray's path. The ray_color function calculates the color
//              as it inetracts with the scene. This method supports recursion to handle
//              multiple bounces of rays, simulating reflection/refractions/emitted light
//              and light scattering
*/
use crate::hittable::*;
use crate::vec3::*;

#[derive(Copy, Clone)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    pub tm: f32,
}
impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f32) -> Self {
        Ray {
            orig: origin,
            dir: direction,
            tm: time,
        }
    }
    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    pub fn time(&self) -> f32 {
        self.tm
    }
    pub fn at(&self, t: f32) -> Point3 {
        self.orig + self.dir * t
    }
}

pub fn ray_color(r: Ray, background: &Color, world: &impl Hittable, depth: i32) -> Color {
    let mut rec = HitRecord::void();

    if depth <= 0 {
        return Color::new();
    }

    if !world.hit(&r, 0.001, f32::INFINITY, &mut rec) {
        return *background;
    }
    
    let mut scattered = Ray::new(Point3::new(), Vec3::new(), 0.);
    let mut attenuation = Color::new();
    let emitted = rec.mat_ptr.emitted(rec.u, rec.v, &rec.p);

    if !rec
        .mat_ptr
        .scatter(r, rec, &mut attenuation, &mut scattered)
    {
        return emitted;
    }

    emitted + attenuation * ray_color(scattered, background, world, depth - 1)
}