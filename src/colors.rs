/*
// Description: This file provides functions to output the color of pixels. 
//              The write_color function writes a pixel's color to the std
//              output, adjusting the number of samples per pixel to perform
//              gamma correction. The write_color_to_write function writes a  
//              pixel's color to a specified writer destination implmenting 
//              the 'Write' trait. 
*/

use crate::utils::clamp;
use crate::vec3::*;
use std::io::stdout;
use std::io::{Error, Write};

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    write_color_to_writer(&mut stdout(), pixel_color, samples_per_pixel as usize).unwrap();
}

pub fn write_color_to_writer<W: Write>(
    writer: &mut W,
    pixel_color: Color,
    samples_per_pixel: usize,
) -> Result<(), Error> {
    let (r, g, b) = pixel_color.into();
    let scale = 1. / samples_per_pixel as f32;
    let (r, g, b) = ((scale * r).sqrt(), (scale * g).sqrt(), (scale * b).sqrt());
    writeln!(
        writer,
        "{} {} {}",
        (256. * clamp(r, 0., 0.999)) as u8,
        (256. * clamp(g, 0., 0.999)) as u8,
        (256. * clamp(b, 0., 0.999)) as u8,
    )
}