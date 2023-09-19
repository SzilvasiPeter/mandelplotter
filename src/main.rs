extern crate num_complex;
use num_complex::Complex;

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
    let c = Complex::new(-0.67, 0.37);
    let max_iterations = 1000;

    if mandelbrot_set(c, max_iterations) {
        println!("The complex number {:?} is in the Mandelbrot set.", c);
    } else {
        println!("The complex number {:?} is not in the Mandelbrot set.", c);
    }
}
