/*
// Description: This file defines rectangular plane structures (xy, xz, yz).
//              All structures implement a bounding box calculation and ray
//              intersection method. Quadrilateral defines a generic shape,
//              defined by 4 points. Xyrect contains fields for the x and y 
//              ranges, as well as the z-coordinate as k. Xzrect contains 
//              fields for the x and z ranges, and y-coordinate as k. 
//              Yzrect contains fields for the y and z ranges, and x-
//              coordinate as k.
*/
use crate::{aabb::*, material::*, hittable::*, ray::*, vec3::*};
use std::sync::Arc;

fn point_in_quadrilateral(pointz: f32, pointy: f32, quad: &Quadrilateral) -> bool {
  let mut crossings = 0;
  let vertices = [(quad.z0, quad.y0), (quad.z1, quad.y1), (quad.z2, quad.y2), (quad.z3, quad.y3)]; 
  let mut j = 3;
  for i in 0..4 {
      let p1 = vertices[i];
      let p2 = vertices[j];

      // Check if pointy is between p1.y and p2.y
      if (p1.1 > pointy) != (p2.1 > pointy) {
          // Compute the z coordinate of the intersection
          let intersect_z = (p2.0 - p1.0) * (pointy - p1.1) / (p2.1 - p1.1) + p1.0;
          //let intersect_z = p1.0 + (pointy - p1.1) * (p2.0 - p1.0) / (p2.1 - p1.1);
          if pointz < intersect_z {
              crossings += 1;
          }
      }
      j = i;
  }
  crossings % 2 != 0
}

/* 
****WARNING****
Quadrilaterals need to be defined in a clock-wise or counter-clockwise sequence of points (CANNOT skip around)
The quadrilaterals will only show as a Y-Z plane, with k as the x-depth
*/
pub struct Quadrilateral {
  mp: Arc<dyn Material>,
  z0: f32,
  z1: f32, 
  z2: f32, 
  z3: f32,
  y0: f32,
  y1: f32,
  y2: f32, 
  y3: f32,
  minz: f32, 
  miny: f32,
  maxz: f32, 
  maxy: f32,
  k: f32,
}
impl Quadrilateral {
  pub fn new() -> Self {
    Self {
      mp: Arc::new(Lambertian::from(Color::new())),
      z0: 0.,
      z1: 0.,
      z2: 0.,
      z3: 0.,
      y0: 0.,
      y1: 0.,
      y2: 0.,
      y3: 0.,
      minz: 0., 
      miny: 0.,
      maxz: 0., 
      maxy: 0.,
      k: 0.,
    }
  }
  pub fn from(pair0: (f32, f32), pair1: (f32, f32), pair2: (f32, f32), pair3: (f32, f32), k: f32, mat: Arc<dyn Material>) -> Self {
    let comp1: f32 = pair0.0.min(pair1.0);
    let comp2: f32 = pair2.0.min(pair3.0);
    let min_z: f32 = comp1.min(comp2);
    let comp3: f32 = pair0.1.min(pair1.1);
    let comp4: f32 = pair2.1.min(pair3.1);
    let min_y: f32 = comp3.min(comp4);
    let comp5: f32 = pair0.0.max(pair1.0);
    let comp6: f32 = pair2.0.max(pair3.0);
    let max_z: f32 = comp5.max(comp6);
    let comp7: f32 = pair0.1.max(pair1.1);
    let comp8: f32 = pair2.1.max(pair3.1);
    let max_y: f32 = comp7.min(comp8);
    let z0 = pair0.0;
    let z1 = pair1.0;
    let z2 = pair2.0;
    let z3 = pair3.0;
    let y0 = pair0.1;
    let y1 = pair1.1;
    let y2 = pair2.1;
    let y3 = pair3.1;
    
    Self {
      z0,
      z1,
      z2,
      z3,
      y0,
      y1,
      y2,
      y3,
      k,
      minz: min_z, 
      miny: min_y,
      maxz: max_z, 
      maxy: max_y,
      mp: mat,
    }
  }
}


impl Hittable for Quadrilateral {
  fn bounding_box(&self, _time0: f32, _time1: f32, output_box: &mut Bb) -> bool {
    let comp1: f32 = self.z0.min(self.z1);
    let comp2: f32 = self.z2.min(self.z3);
    let minz: f32 = comp1.min(comp2);
    let comp3: f32 = self.y0.min(self.y1);
    let comp4: f32 = self.y2.min(self.y3);
    let miny: f32 = comp3.min(comp4);
    let comp5: f32 = self.z0.max(self.z1);
    let comp6: f32 = self.z2.max(self.z3);
    let maxz: f32 = comp5.max(comp6);
    let comp7: f32 = self.y0.max(self.y1);
    let comp8: f32 = self.y2.max(self.y3);
    let maxy: f32 = comp7.min(comp8);
    *output_box = Bb::from(
      &Point3::from(self.k - 0.0001, miny, minz),
      &Point3::from(self.k + 0.0001, maxy, maxz),
    );
    true
  }
  fn hit<'a>(&'a self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a>) -> bool {
    let t = (self.k - r.origin().x()) / r.direction().x();
    
    if t < t_min || t > t_max {
      return false;
    }
    let y = r.origin().y() + t * r.direction().y();
    let z = r.origin().z() + t * r.direction().z();

    if !point_in_quadrilateral(z, y, self){
      return false;
    }
    // if z < self.minz || z > self.maxz || y < self.miny || y > self.maxy {
    //   return false;
    // }
  

    // ********** WARNING *************
    // Possibly buggy wiht non-axis aligned rectangles, and numerically unstable at edges
    let s = (z - self.minz) / (self.maxz - self.minz); // Simplified linear interpolation
    let t = (y - self.miny) / (self.maxy - self.miny); // Simplified linear interpolation

    rec.u = (1.0 - t) * ((1.0 - s) * 0.0 + s * 1.0) + t * ((1.0 - s) * 0.0 + s * 1.0);
    rec.v = (1.0 - t) * ((1.0 - s) * 0.0 + s * 0.0) + t * ((1.0 - s) * 1.0 + s * 1.0);
    // *******************************

    rec.t = t;

    let outward_normal = Vec3::from(0., 0., 1.);
    rec.set_face_normal(*r, outward_normal);
    rec.mat_ptr = &*self.mp;
    rec.p = r.at(t);

    true
  }
}

pub struct XyRect {
  mp: Arc<dyn Material>,
  x0: f32,
  x1: f32,
  y0: f32,
  y1: f32,
  k: f32,
}

impl XyRect {
  pub fn new() -> Self {
    Self {
      mp: Arc::new(Lambertian::from(Color::new())),
      x0: 0.,
      x1: 0.,
      y0: 0.,
      y1: 0.,
      k: 0.,
    }
  }
  pub fn from(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, mat: Arc<dyn Material>) -> Self {
    Self {
      x0,
      x1,
      y0,
      y1,
      k,
      mp: mat,
    }
  }
}

impl Hittable for XyRect {
  fn bounding_box(&self, _time0: f32, _time1: f32, output_box: &mut Bb) -> bool {
    *output_box = Bb::from(
      &Point3::from(self.x0, self.y0, self.k - 0.0001),
      &Point3::from(self.x1, self.y1, self.k + 0.0001),
    );
    true
  }
  fn hit<'a>(&'a self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a>) -> bool {
    let t = (self.k - r.origin().z()) / r.direction().z();
    if t < t_min || t > t_max {
      return false;
    }
    let x = r.origin().x() + t * r.direction().x();
    let y = r.origin().y() + t * r.direction().y();
    if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
      return false;
    }

    rec.u = (x - self.x0) / (self.x1 - self.x0);
    rec.v = (y - self.y0) / (self.y1 - self.y0);

    rec.t = t;

    let outward_normal = Vec3::from(0., 0., 1.);
    rec.set_face_normal(*r, outward_normal);
    rec.mat_ptr = &*self.mp;
    rec.p = r.at(t);

    true
  }
}

pub struct XzRect {
  mp: Arc<dyn Material>,
  x0: f32,
  x1: f32,
  z0: f32,
  z1: f32,
  k: f32,
}

impl XzRect {
  pub fn new() -> Self {
    Self {
      mp: Arc::new(Lambertian::from(Color::new())),
      x0: 0.,
      x1: 0.,
      z0: 0.,
      z1: 0.,
      k: 0.,
    }
  }
  pub fn from(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, mat: Arc<dyn Material>) -> Self {
    Self {
      x0,
      x1,
      z0,
      z1,
      k,
      mp: mat,
    }
  }
}

impl Hittable for XzRect {
  fn bounding_box(&self, _time0: f32, _time1: f32, output_box: &mut Bb) -> bool {
    *output_box = Bb::from(
      &Point3::from(self.x0, self.k - 0.0001, self.z0),
      &Point3::from(self.x1, self.k + 0.0001, self.z1),
    );
    true
  }
  fn hit<'a>(&'a self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a>) -> bool {
    let t = (self.k - r.origin().y()) / r.direction().y();
    if t < t_min || t > t_max {
      return false;
    }
    let x = r.origin().x() + t * r.direction().x();
    let z = r.origin().z() + t * r.direction().z();
    if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
      return false;
    }

    rec.u = (x - self.x0) / (self.x1 - self.x0);
    rec.v = (z - self.z0) / (self.z1 - self.z0);

    rec.t = t;

    let outward_normal = Vec3::from(0., 0., 1.);
    rec.set_face_normal(*r, outward_normal);
    rec.mat_ptr = &*self.mp;
    rec.p = r.at(t);

    true
  }
}

pub struct YzRect {
  mp: Arc<dyn Material>,
  y0: f32,
  y1: f32,
  z0: f32,
  z1: f32,
  k: f32,
}

impl YzRect {
  pub fn new() -> Self {
    Self {
      mp: Arc::new(Lambertian::from(Color::new())),
      y0: 0.,
      y1: 0.,
      z0: 0.,
      z1: 0.,
      k: 0.,
    }
  }
  pub fn from(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, mat: Arc<dyn Material>) -> Self {
    Self {
      y0,
      y1,
      z0,
      z1,
      k,
      mp: mat,
    }
  }
}

impl Hittable for YzRect {
  fn bounding_box(&self, _time0: f32, _time1: f32, output_box: &mut Bb) -> bool {
    *output_box = Bb::from(
      &Point3::from(self.k - 0.0001, self.y0, self.z0),
      &Point3::from(self.k + 0.0001, self.y1, self.z1),
    );
    true
  }
  fn hit<'a>(&'a self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a>) -> bool {
    let t = (self.k - r.origin().x()) / r.direction().x();
    if t < t_min || t > t_max {
      return false;
    }
    let y = r.origin().y() + t * r.direction().y();
    let z = r.origin().z() + t * r.direction().z();
    if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
      return false;
    }

    rec.u = (y - self.y0) / (self.y1 - self.y0);
    rec.v = (z - self.z0) / (self.z1 - self.z0);

    rec.t = t;

    let outward_normal = Vec3::from(0., 0., 1.);
    rec.set_face_normal(*r, outward_normal);
    rec.mat_ptr = &*self.mp;
    rec.p = r.at(t);

    true
  }
}