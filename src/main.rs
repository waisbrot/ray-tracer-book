mod tuple;
use std::error::Error;

use book_renderer::{canvas::Canvas, color::Color};


fn main() -> Result<(), Box<dyn Error>>{
    println!("Hello, world!");
    let mut c = Canvas::new(50, 25);
    for x in 0..50 {
        c.pixels[(10, x)] = Color::new(1.0, 0.0, 0.0);
    }
    c.write_png("/tmp/test.png")?;
    Ok(())
}
