use std::error::Error;
use book_renderer::color::*;
use proptest::prelude::*;

#[test]
fn test_book_color_new() {
    let c = Color::new(-0.5, 0.4, 1.7);
    assert_eq!(c.red, -0.5);
    assert_eq!(c.green, 0.4);
    assert_eq!(c.blue, 1.7)
}

#[test]
fn test_book_color_add() {
    let c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);
    assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
}

#[test]
fn test_book_color_sub() {
    let c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);
    assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
}

#[test]
fn test_book_color_mul() {
    let c1 = Color::new(1.0, 0.2, 0.4);
    let c2 = Color::new(0.9, 1.0, 0.1);
    assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
}