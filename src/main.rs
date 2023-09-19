extern crate num_complex;
extern crate image;

use num_complex::Complex;
use image::RgbImage;
use image::Rgb;

fn mandelbrot_set(c: Complex<f64>, max_iterations: u32) -> bool {
    let mut z = Complex::new(0.0, 0.0);

    for _i in 0..max_iterations {
        z = z * z + c;

        if z.norm() > 2.0 {
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

    let mut img = RgbImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            // Map pixel coordinates to complex plane coordinates
            let cx = center_x + (x as f64 - width as f64 / 2.0) / (width as f64 / 2.0) * zoom;
            let cy = center_y + (y as f64 - height as f64 / 2.0) / (height as f64 / 2.0) * zoom;

            let c = Complex::new(cx, cy);

            if mandelbrot_set(c, max_iterations) {
                // Set pixel to black
                img.put_pixel(x, y, Rgb([0, 0, 0]));
            } else {
                // Set pixel to white
                img.put_pixel(x, y, Rgb([255, 255, 255]));
            }
        }
    }

    img.save("mandelbrot.png").unwrap();
}
