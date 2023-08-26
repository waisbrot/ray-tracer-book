use std::f64::consts::PI;

use book_renderer::{canvas::Canvas, tuple::Tuple, util::Float, color::GREEN, matrix::Matrix};
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, default_value = "/tmp/clock.png")]
    outfile: String,
}

fn draw_dot(canvas: &mut Canvas, point: &Tuple) {
    let x = ((point.x + canvas.width as Float/2.0) as i64).clamp(0, canvas.width as i64) as usize;
    let y = ((point.y + canvas.height as Float/2.0) as i64).clamp(0, canvas.height as i64) as usize;
    for i in x-2..x+2 {
        for j in y-2..y+2 {
            canvas[(i,j)] = GREEN;
        }
    }
}

fn main() {
    let args = Args::parse();
    let mut canvas = Canvas::new(500, 500);
    let noon = Tuple::new_point(0.0, -100.0, 0.0);
    for i in 0..12 {
        let rads = (PI / 6.0) * (i as f64);
        let time = Matrix::rotation_z(rads) * noon;
        draw_dot(&mut canvas, &time);
    }

    canvas.write_png(&args.outfile).unwrap();
}