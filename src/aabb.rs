/*
// Description: This file provides an axis-aligned bounding box struct for use
//              in ray tracing. The bb struct representes a bounding box in 3D
//              space defined by min and max points. There are methods provided to
//              generate the bounding box given two sets of points, and also to 
//              determine if a ray intersects the bounding box within a given range
//              of t-values. 
*/

use crate::*;

#[derive(Copy, Clone, Default)]
pub struct Bb {
    //point3 is a vec3 struct
  min: Point3,
  max: Point3,
}
pub fn surrounding_box(box0: &Bb, box1: &Bb) -> Bb {
  let small_coords = Point3::from(
    min(box0.min().x(), box1.min().x()),
    min(box0.min().y(), box1.min().y()),
    min(box0.min().z(), box1.min().z()),
  );

  let large_coords = Point3::from(
    max(box0.min().x(), box1.min().x()),
    max(box0.min().y(), box1.min().y()),
    max(box0.min().z(), box1.min().z()),
  );

  Bb::from(&small_coords, &large_coords)
}
impl Bb {
  pub fn new() -> Self {
    Self {
      min: Point3::new(),
      max: Point3::new(),
    }
  }
  pub fn from(a: &Point3, b: &Point3) -> Self {
    Self {
      min: *a,
      max: *b,
    }
  }
  pub fn min(&self) -> Point3 {
    self.min
  }
  pub fn max(&self) -> Point3 {
    self.max
  }
  pub fn hit(&self, r: &Ray, mut t_min: f32, mut t_max: f32) -> bool {
    for i in 0..3 {
      let inv_d = 1.0 / r.direction()[i];

      let mut t0 = (self.min()[i] - r.origin()[i]) * inv_d;
      let mut t1 = (self.max()[i] - r.origin()[i]) * inv_d;

      if inv_d < 0.0 {
        let prev_t0 = t0;
        t0 = t1;
        t1 = prev_t0;
      }
      t_min = if t0 > t_min { t0 } else { t_min };
      t_max = if t1 < t_max { t1 } else { t_max };

      if t_max <= t_min {
        return false;
      }
    }
    true
  }
}

fn min(a: f32, b: f32) -> f32 {
  if a == std::f32::NAN {
    return b;
  } else if b == std::f32::NAN {
    return a;
  }
  if a < b {
    return a;
  } else {
    return b;
  }
}
fn max(a: f32, b: f32) -> f32 {
  if a == std::f32::NAN {
    return b;
  } else if b == std::f32::NAN {
    return a;
  }
  if a > b {
    return a;
  } else {
    return b;
  }
}