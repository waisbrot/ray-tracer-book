use book_renderer::{tuple::Point, canvas::Canvas, sphere::Sphere, matrix::Matrix, ray::Ray, color::{Color, BLACK}, intersectable::Intersectable, intersection::Intersection};
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, default_value = "0,0,-20", value_parser = Point::parse_point)]
    camera: Point,

    #[arg(long, default_value = "0,0,-10", value_parser = Point::parse_vector)]
    sphere: Point,

    #[arg(long, default_value = "0")]
    canvas_z: f64,

    #[arg(long, default_value = "100")]
    canvas_size: usize,

    #[arg(long, default_value = "/tmp/canvas.png")]
    outfile: String,
}

fn main() {
    let args = Args::parse();
    let mut canvas = Canvas::new(args.canvas_size, args.canvas_size);
    let mut shape = Sphere::new_unit();
    let wall_size = 8.0;
    let canvas_pixels = args.canvas_size as f64;
    let pixel_size = wall_size / canvas_pixels;
    let half_wall = wall_size / 2.0;
    shape.set_transform(Matrix::translation(args.sphere.x, args.sphere.y, args.sphere.z) * Matrix::scaling(2.0, 1.0, 1.0));
    for canvas_y in 0..args.canvas_size {
        let world_y = half_wall - pixel_size * (canvas_y as f64);
        for canvas_x in 0..args.canvas_size {
            let world_x = -half_wall + pixel_size * (canvas_x as f64);
            let position = Point::new_point(world_x, world_y, args.canvas_z as f64);
            let origin = args.camera.clone();
            let ray = Ray::new(origin, &position - &origin);
            let intersections = shape.intersections(&ray);
            match Intersection::hit(&intersections) {
                Some(_) => canvas[(canvas_x, canvas_y)] = Color::new(50.0, 0.0, 0.0),
                None => canvas[(canvas_x, canvas_y)] = BLACK,
            }
        }
    }
    canvas.write_png(args.outfile).unwrap();
}