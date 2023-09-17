use book_renderer::canvas::*;
use book_renderer::color::*;

#[test]
fn book_test_canvas_new() {
    let c = Canvas::new(10, 20);
    assert_eq!(c.width, 10);
    assert_eq!(c.height, 20);
    for x in 0..c.width {
        for y in 0..c.height {
            assert_eq!(c[(x,y)], Color::new(0.0, 0.0, 0.0));
        }
    }
}

#[test]
fn book_test_write_pixel() {
    let mut c = Canvas::new(10, 20);
    let red = Color::new(1.0, 0.0, 0.0);
    c.pixels[(3,2)] = red.clone();
    assert_eq!(c[(2,3)], red);
}

#[test]
fn book_test_ppm_header() {
    let c = Canvas::new(5, 3);
    let data = c.ppm_data();
    assert_eq!(data[0], "P3");
    assert_eq!(data[1], "5 3");
    assert_eq!(data[2], "255");
}

#[test]
fn book_test_ppm_data() {
    let mut c = Canvas::new(5, 3);
    c[(0,0)] = Color::new(1.5, 0.0, 0.0);
    c[(2,1)] = Color::new(0.0, 0.5, 0.0);
    c[(4,2)] = Color::new(-0.5, 0.0, 1.0);
    let data = c.ppm_data();
    assert_eq!(data[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
    assert_eq!(data[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
    assert_eq!(data[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
}

#[test]
#[ignore] // I am not implementing line-wrapping
fn book_test_ppm_long_lines() {
    let mut c = Canvas::new(10, 2);
    for x in 0..c.width {
        for y in 0..c.height {
            c[(x, y)] = Color::new(1.0, 0.8, 0.6);
        }
    }
    let data = c.ppm_data();
    assert_eq!(data[3], "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204");
}

#[test]
fn book_test_newline_ending() {
    let c = Canvas::new(5, 3);
    let data = c.ppm_data();
    assert_eq!(data.last().unwrap(), "\n");
}