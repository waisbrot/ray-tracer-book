use book_renderer::{color::Color, material::Material, tuple::Tuple, light::Light};

#[test]
fn test_book_default_material() {
    let m = Material::default();
    assert_eq!(m.color, Color::white(1.0));
    assert_eq!(m.ambient, 0.1);
    assert_eq!(m.diffuse, 0.9);
    assert_eq!(m.specular, 0.9);
    assert_eq!(m.shininess, 200.0);
}

#[test]
fn test_book_light_eye_between_light_and_surface() {
    let m = Material::default();
    let position = Tuple::origin_point();
    let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
    let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
    let light = Light::new_point(Tuple::new_point(0.0, 0.0, -10.0), Color::white(1.0));
    let result = m.light(&light, &position, &eyev, &normalv);
    assert_eq!(result, Color::white(1.9));
}

#[test]
fn test_book_light_eye_between_light_and_surface_eye_offset_45() {
    let m = Material::default();
    let position = Tuple::origin_point();
    let eyev = Tuple::new_vector(0.0, 2.0_f64.sqrt()/2.0, -2.0_f64.sqrt()/2.0);
    let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
    let light = Light::new_point(Tuple::new_point(0.0, 0.0, -10.0), Color::white(1.0));
    let result = m.light(&light, &position, &eyev, &normalv);
    assert_eq!(result, Color::white(1.0));
}

#[test]
fn test_book_light_eye_between_light_and_surface_light_offset_45() {
    let m = Material::default();
    let position = Tuple::origin_point();
    let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
    let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
    let light = Light::new_point(Tuple::new_point(0.0, 10.0, -10.0), Color::white(1.0));
    let result = m.light(&light, &position, &eyev, &normalv);
    assert_eq!(result, Color::white(0.7364));
}

#[test]
fn test_book_light_eye_and_surface_90_from_each_other() {
    let m = Material::default();
    let position = Tuple::origin_point();
    let eyev = Tuple::new_vector(0.0, -2.0_f64.sqrt()/2.0, -2.0_f64.sqrt()/2.0);
    let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
    let light = Light::new_point(Tuple::new_point(0.0, 10.0, -10.0), Color::white(1.0));
    let result = m.light(&light, &position, &eyev, &normalv);
    assert_eq!(result, Color::white(1.6364));
}

#[test]
fn test_book_light_light_behind_surface() {
    let m = Material::default();
    let position = Tuple::origin_point();
    let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
    let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
    let light = Light::new_point(Tuple::new_point(0.0, 0.0, 10.0), Color::white(1.0));
    let result = m.light(&light, &position, &eyev, &normalv);
    assert_eq!(result, Color::white(0.1));
}
