/*
// Description: This file contains several texture types to be used in the ray tracing application.
//              The texture trait is an interface for different texture implementations, it requires
//              a value method to give the color at a given point. The solid color texture returns a
//              constant color, it provides constructors for creating colors from individual rgb values
//              or a color object. The checkertexture generates a pattern which alternates based on 3d 
//              position. The imagetexture loads in and uses an image as a texture, mapping them onto
//              3d objects.
*/

use crate::{utils::*, vec3::*};
use stb_image::{image::load, image::LoadResult::*};
use std::path::Path;
use std::sync::Arc;

pub trait Texture: Send + Sync {
  fn value(&self, u: f32, v: f32, p: &Point3) -> Color;
}
#[derive(Clone)]
pub struct SolidColor {
  color_value: Color,
}
impl SolidColor {
  pub fn new() -> Self {
    Self {
      color_value: Color::new(),
    }
  }
  pub fn from(c: Color) -> Self {
    Self { color_value: c }
  }
  pub fn fromrgb(r: f32, g: f32, b: f32) -> Self {
    Self {
      color_value: Color::from(r, g, b),
    }
  }
}

impl Texture for SolidColor {
  fn value(&self, _u: f32, _v: f32, _p: &Point3) -> Color {
    self.color_value
  }
}

pub struct CheckerTexture {
  odd: Arc<dyn Texture>,
  even: Arc<dyn Texture>,
}

impl CheckerTexture {
  pub fn from_texture(_even: Arc<dyn Texture>, _odd: Arc<dyn Texture>) -> Self {
    Self {
      odd: _odd,
      even: _even,
    }
  }
  pub fn from_colors(c1: Color, c2: Color) -> Self {
    Self {
      even: Arc::new(SolidColor::from(c1)),
      odd: Arc::new(SolidColor::from(c2)),
    }
  }
}

impl Texture for CheckerTexture {
  fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
    let sines = (10. * p.x()).sin() * (10. * p.y()).sin() * (10. * p.z()).sin();
    if sines < 0. {
      return self.odd.value(u, v, p);
    } else {
      return self.even.value(u, v, p);
    }
  }
}pub struct ImageTexture {
  pub bytes_per_pixel: usize,
  data: Vec<u8>,
  width: usize,
  height: usize,
  bytes_per_scanline: usize,
}

impl ImageTexture {
  pub fn new() -> Self {
    Self {
      data: vec![],
      bytes_per_pixel: 3,
      width: 0,
      height: 0,
      bytes_per_scanline: 0,
    }
  }
  pub fn from(filename: &str) -> Self {
    let bytes_per_pixel = 3;

    if !Path::new(filename).exists() {
      eprintln!("ERROR: file `{}` doesn't exist !", filename);
      return Self {
        bytes_per_pixel,
        data: vec![],
        width: 0,
        height: 0,
        bytes_per_scanline: 0,
      };
    }

    let (width, height, data) = match load(filename) {
      ImageU8(v) => (v.width, v.height, v.data),
      _ => {
        eprintln!("ERROR: Could not load texture image file {}", filename);
        (0, 0, vec![])
      }
    };
    let bytes_per_scanline = bytes_per_pixel * width;
    Self {
      bytes_per_pixel,
      data,
      width,
      height,
      bytes_per_scanline,
    }
  }
}

impl Texture for ImageTexture {
  fn value(&self, u: f32, v: f32, _p: &Point3) -> Color {
    if self.data.len() == 0 {
      return Color::new();
    }

    let uu = clamp(u, 0., 1.);
    let vv = 1. - clamp(v, 0., 1.);

    let mut i = (uu * self.width as f32) as usize;
    let mut j = (vv * self.height as f32) as usize;

    if i >= self.width {
      i = self.width - 1;
    }
    if j >= self.height {
      j = self.height - 1;
    }

    let color_scale = 1. / 255.;
    let offset = j * self.bytes_per_scanline + i * self.bytes_per_pixel;
    let pixel = &self.data[offset..];

    Color::from(
      color_scale * pixel[0] as f32,
      color_scale * pixel[1] as f32,
      color_scale * pixel[2] as f32,
    )
  }
}