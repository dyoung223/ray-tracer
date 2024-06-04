/*
// Description: This file provides a canvas struct for rendering pixel data.
//              There is a method for both sequential and parallel rendering 
//              as well as outputting the image. The canvas struct manages
//              pixel data and supports parallel computation. There is a 
//              method for a serial render, a parallel render using the Rayon
//              library, and another parallel implementation with a visual 
//              progress tracking bar during runtime. These functions take the
//              the pixel computing function in as a parameter. There are also
//              various helper methods and methods to properly write out the
//              image in ppm format. 
*/


use crate::colors::write_color_to_writer;
use crate::vec3::Color;
use rayon::prelude::*;

use std::io::stdout;
use std::io::Write;

//use std::sync::{Mutex , Condvar , Arc , atomic::{Ordering , AtomicUsize}};

pub struct Canvas {
    pixels: Box<[Color]>,
    samples_per_pixel: usize,
    xsize: usize,
    ysize: usize,
}

//const UPDATE_INTEVAL: usize = 1024;

#[inline]
fn index_to_xy(xsize: usize , index: usize) -> (usize , usize){
    (index % xsize , index / xsize)
}


impl Canvas {
    pub fn from_fn<F>(x: usize, y: usize, samples_per_pixel: usize, mut f: F) -> Self
    where
        F: FnMut(usize, usize) -> Color,
    {
        let size = x.checked_mul(y).unwrap();
        
        //let mut pixels = Region::<Color>::new(size);
        let mut pixels = (vec![Color::default(); size]).into_boxed_slice();
        let mut pix_iter = pixels.iter_mut();
        for y in 0..y {
            for x in 0..x {
                *pix_iter.next().unwrap() = f(x, y);

            }
        }
        
        

        Self {
            samples_per_pixel,
            pixels,
            xsize: x,
            ysize: y,
        }
    }
    pub fn from_fn_parallel<F>(x: usize, y: usize, samples_per_pixel: usize, f: F) -> Self
    where
        F: Fn(usize, usize) -> Color + Send + Sync,
    {
        let size = x.checked_mul(y).unwrap();
        let mut pixels = Vec::with_capacity(size);
        pixels.par_extend((0..size).into_par_iter().map(|idx| {
            let (x , y) = index_to_xy(x , idx);
            f(x , y)
        }));
        Self {
            samples_per_pixel,
            pixels: pixels.into_boxed_slice(),
            xsize: x,
            ysize: y,
        }
    }
    
    pub fn write_pixels(&self) {
        let stdout = stdout();
        let mut locked = stdout.lock();
        self.write_pixels_to_writer(&mut locked).unwrap();
    }
    pub fn write_pixels_to_writer<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        self.iter_pixels()
            .try_for_each(|&p| write_color_to_writer(writer, p, self.samples_per_pixel))
    }
    pub fn write_header(&self) {
        self.write_header_to_writer(&mut stdout()).unwrap();
    }
    pub fn write_header_to_writer<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        writeln!(w, "P3\n{} {}\n255", self.xsize, self.ysize)
    }

    pub fn iter_pixels(&self) -> impl Iterator<Item = &'_ Color> + '_ {
        self.pixels.chunks(self.xsize).rev().flatten()
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.xsize, self.ysize)
    }
    pub fn get_pixels(&self) -> &[Color] {
        &*self.pixels
    }
}

