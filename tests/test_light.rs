use book_renderer::{tuple::Tuple, color::Color, light::Light};

#[test]
fn book_test_light_init() {
    let intensity = Color::new(1.0, 1.0, 1.0);
    let position = Tuple::new_point(0.0, 0.0, 0.0);
    let light = Light::new_point(position, intensity);
    assert_eq!(light.position, Tuple::new_point(0.0, 0.0, 0.0));
}