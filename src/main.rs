extern crate num_complex;
extern crate image;
extern crate rayon;

use num_complex::Complex;
use image::{RgbImage, Rgb};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

fn mandelbrot_set(c: Complex<f64>, max_iterations: u32) -> bool {
    let mut z = Complex::new(0.0, 0.0);

    for _i in 0..max_iterations {
        z = z * z + c;

        if z.norm_sqr() > 4.0 {
            return false;
        }
    }
    
    return true;
}

fn main() {
    let width = 800;
    let height = 800;
    let max_iterations = 2000;
    let center_x = -0.5;
    let center_y = 0.0;
    let zoom = 2.0;

    let img = Arc::new(Mutex::new(RgbImage::new(width, height)));

    (0..height).into_par_iter().for_each(|y| {
        for x in 0..width {
            let cx = center_x + (x as f64 - width as f64 / 2.0) / (width as f64 / 2.0) * zoom;
            let cy = center_y + (y as f64 - height as f64 / 2.0) / (height as f64 / 2.0) * zoom;

            let c = Complex::new(cx, cy);

            let color = if mandelbrot_set(c, max_iterations) {
                Rgb([0, 0, 0]) // Set pixel to black
            } else {
                Rgb([255, 255, 255]) // Set pixel to white
            };

            img.lock().unwrap().put_pixel(x, y, color);
        }
    });

    // Save the image as a PNG file
    img.lock()
        .unwrap()
        .save("mandelbrot.png")
        .expect("Failed to save image");
}
