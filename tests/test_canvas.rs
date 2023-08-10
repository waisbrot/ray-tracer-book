use book_renderer::canvas::*;
use book_renderer::color::*;

#[test]
fn book_test_canvas_new() {
    let c = Canvas::new(10, 20);
    assert_eq!(c.width, 10);
    assert_eq!(c.height, 20);
    for x in 0..c.width {
        for y in 0..c.height {
            assert_eq!(c.pixels[x][y], Color::new(0.0, 0.0, 0.0));
        }
    }
}

#[test]
fn book_test_write_pixel() {
    let mut c = Canvas::new(10, 20);
    let red = Color::new(1.0, 0.0, 0.0);
    c.pixels[2][3] = red.clone();
    assert_eq!(c.pixels[2][3], red);
}
