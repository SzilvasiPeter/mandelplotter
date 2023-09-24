extern crate sdl2;
extern crate num_complex;
extern crate rayon;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use num_complex::Complex;
use rayon::prelude::*;
use std::time::Duration;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const MAX_ITERATIONS: u32 = 1000;

const MANDELBROT_X_MIN: f64 = -2.0;
const MANDELBROT_X_MAX: f64 = 1.0;
const MANDELBROT_Y_MIN: f64 = -1.5;
const MANDELBROT_Y_MAX: f64 = 1.5;

fn mandelbrot(c: Complex<f64>) -> u32 {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let mut iteration = 0;

    while z.norm_sqr() <= 4.0 && iteration < MAX_ITERATIONS {
        z = z * z + c;
        iteration += 1;
    }

    iteration
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Mandelbrot Set", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut zoom_factor = 1.0;
    let mut pan_x = 0.0;
    let mut pan_y = 0.0;

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    repeat: false,
                    ..
                } => {
                    // Zoom in by reducing the zoom factor
                    zoom_factor /= 0.9;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    repeat: false,
                    ..
                } => {
                    // Zoom out by increasing the zoom factor
                    zoom_factor *= 0.9;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    repeat: false,
                    ..
                } => {
                    // Pan left
                    pan_x -= 0.1 / zoom_factor;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    repeat: false,
                    ..
                } => {
                    // Pan right
                    pan_x += 0.1 / zoom_factor;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    repeat: false,
                    ..
                } => {
                    // Pan up
                    pan_y -= 0.1 / zoom_factor;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    repeat: false,
                    ..
                } => {
                    // Pan down
                    pan_y += 0.1 / zoom_factor;
                }
                _ => {}
            }
        }

        // Clear the canvas
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        // Parallelize the calculation and drawing of the Mandelbrot set
        let pixels: Vec<(u32, u32, Color)> = (0..WINDOW_WIDTH)
            .into_par_iter()
            .flat_map(|px| {
                (0..WINDOW_HEIGHT).into_par_iter().map(move |py| {
                    let x = ((px as f64 - WINDOW_WIDTH as f64 / 2.0) / (zoom_factor * WINDOW_WIDTH as f64) + pan_x)
                        * (MANDELBROT_X_MAX - MANDELBROT_X_MIN)
                        + (MANDELBROT_X_MAX + MANDELBROT_X_MIN) / 2.0;
                    let y = ((py as f64 - WINDOW_HEIGHT as f64 / 2.0) / (zoom_factor * WINDOW_HEIGHT as f64) + pan_y)
                        * (MANDELBROT_Y_MAX - MANDELBROT_Y_MIN)
                        + (MANDELBROT_Y_MAX + MANDELBROT_Y_MIN) / 2.0;

                    let c = Complex::new(x, y);
                    let iteration = mandelbrot(c);
                    
                    // Set color based on the number of iterations
                    let color = Color::RGB(
                        (iteration % 256) as u8,
                        ((iteration / 8) % 256) as u8,
                        ((iteration / 16) % 256) as u8,
                    );

                    (px, py, color)
                })
            })
            .collect();

        // Draw the pixels
        for (px, py, color) in pixels {
            canvas.set_draw_color(color);
            canvas.draw_point((px as i32, py as i32)).unwrap();
        }

        // Present the canvas
        canvas.present();

        // Delay to control frame rate (optional)
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
