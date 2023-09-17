use std::ops::IndexMut;

use book_renderer::{world::World, light::Light, tuple::Tuple, color::Color, matrix::Matrix, ray::Ray, intersection::Intersection};


#[test]
fn test_book_default_world() {
    let mut w = World::default();
    assert_eq!(w.light, Light::new_point(Tuple::new_point(-10.0, 10.0, -10.0), Color::white(1.0)));
    assert_eq!(w.objects[0].transformation(), &Matrix::identity(4));
    assert_eq!(w.objects.index_mut(0).mut_material().diffuse, 0.7);
    assert_eq!(w.objects[0].mut_material().color, Color::new(0.8, 1.0, 0.6));
    assert_eq!(w.objects[1].transformation(), &Matrix::scaling(0.5, 0.5, 0.5));
    assert_eq!(w.objects[1].mut_material().diffuse, 0.9);
    assert_eq!(w.objects[1].mut_material().color, Color::white(1.0));
}

#[test]
fn test_book_intersect_world() {
    let w = World::default();
    let r = Ray::new(Tuple::new_point(0.0, 0.0, -5.0), Tuple::new_vector(0.0, 0.0, 1.0));
    let xs = w.intersections(&r);
    assert_eq!(xs.len(), 4);
    assert_eq!(xs[0].t, 4.0);
    assert_eq!(xs[1].t, 4.5);
    assert_eq!(xs[2].t, 5.5);
    assert_eq!(xs[3].t, 6.0);
}

// pg 95
#[test]
fn test_book_shade_intersection() {
    let w = World::default();
    let r = Ray::new(Tuple::new_point(0.0, 0.0, -5.0), Tuple::new_vector(0.0, 0.0, 1.0));
    let shape = w.objects[0].as_ref();
    let i = Intersection{ t: 4.0, object: shape };
    let comps = i.precompute(&r);
    let c = w.shade_hit(&comps);
    assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
}

// pg 95
#[test]
fn test_book_shade_intersection_internal() {
    let mut w = World::default();
    w.light = Light::new_point(Tuple::new_point(0.0, 0.25, 0.0), Color::white(1.0));
    let r = Ray::new(Tuple::new_point(0.0, 0.0, 0.0), Tuple::new_vector(0.0, 0.0, 1.0));
    let shape = w.objects[1].as_ref();
    let i = Intersection{ t: 0.5, object: shape };
    let comps = i.precompute(&r);
    let c = w.shade_hit(&comps);
    assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
}

// pg 96
#[test]
fn test_book_color_miss() {
    let w = World::default();
    let r = Ray::new(Tuple::new_point(0.0, 0.0, -5.0), Tuple::new_vector(0.0, 1.0, 0.0));
    assert_eq!(w.color_at(&r), Color::white(0.0));
}

// pg 96
#[test]
fn test_book_color_hit() {
    let w = World::default();
    let r = Ray::new(Tuple::new_point(0.0, 0.0, -5.0), Tuple::new_vector(0.0, 0.0, 1.0));
    assert_eq!(w.color_at(&r), Color::new(0.38066, 0.47583, 0.2855));
}

// pg 97
#[test]
fn test_book_color_hit_behind_ray() {
    let mut w = World::default();
    w.objects[0].mut_material().ambient = 1.0;
    w.objects[1].mut_material().ambient = 1.0;
    let r = Ray::new(Tuple::new_point(0.0, 0.0, 0.75), Tuple::new_vector(0.0, 0.0, -1.0));
    assert_eq!(w.color_at(&r), w.objects[1].material().color);
}