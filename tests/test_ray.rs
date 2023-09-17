
use book_renderer::{tuple::Tuple, ray::Ray, sphere::Sphere, matrix::Matrix, intersectable::Intersectable, intersection::Intersection};

#[test]
fn test_book_new_ray() {
    let p = Tuple::new_point(1.0, 0.0, 0.0);
    let v = Tuple::new_vector(0.0, 0.0, 1.0);
    Ray::new(p, v);
}

#[test]
fn test_book_compute_point() {
    let r = Ray::new(Tuple::new_point(2.0, 3.0, 4.0), Tuple::new_vector(1.0, 0.0, 0.0));
    assert_eq!(r.position(&0.0), Tuple::new_point(2.0, 3.0, 4.0));
    assert_eq!(r.position(&1.0), Tuple::new_point(3.0, 3.0, 4.0));
    assert_eq!(r.position(&-1.0), Tuple::new_point(1.0, 3.0, 4.0));
    assert_eq!(r.position(&2.5), Tuple::new_point(4.5, 3.0, 4.0));
}

#[test]
fn test_book_ray_sphere_intersect() {
    let r = Ray::new(Tuple::new_point(0.0, 0.0, -5.0), Tuple::new_vector(0.0, 0.0, 1.0));
    let s = Sphere::new_unit();
    assert!(s.is_intersecting(&r));
    let intersections = s.intersections(&r);
    assert_eq!(s.intersection_count(&r), 2);
    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].t, 4.0);
    assert_eq!(intersections[1].t, 6.0);
}

#[test]
fn test_book_ray_sphere_tangent() {
    let r = Ray::new(Tuple::new_point(0.0, 1.0, -5.0), Tuple::new_vector(0.0, 0.0, 1.0));
    let s = Sphere::new_unit();
    assert!(s.is_intersecting(&r));
    let intersections = s.intersections(&r);
    assert_eq!(s.intersection_count(&r), 1);
    assert_eq!(intersections.len(), 1);
    assert_eq!(intersections[0].t, 5.0);
}

#[test]
fn test_book_ray_sphere_nonintersect() {
    let r = Ray::new(Tuple::new_point(0.0, 2.0, -5.0), Tuple::new_vector(0.0, 0.0, 1.0));
    let s = Sphere::new_unit();
    assert!(!s.is_intersecting(&r));
    let intersections = s.intersections(&r);
    assert_eq!(s.intersection_count(&r), 0);
    assert_eq!(intersections.len(), 0);
}

#[test]
fn test_book_ray_sphere_internal_intersect() {
    let r = Ray::new(Tuple::new_point(0.0, 0.0, 0.0), Tuple::new_vector(0.0, 0.0, 1.0));
    let s = Sphere::new_unit();
    assert!(s.is_intersecting(&r));
    let intersections = s.intersections(&r);
    assert_eq!(s.intersection_count(&r), 2);
    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].t, -1.0);
    assert_eq!(intersections[1].t, 1.0);
}

#[test]
fn test_book_ray_sphere_behind_intersect() {
    let r = Ray::new(Tuple::new_point(0.0, 0.0, 5.0), Tuple::new_vector(0.0, 0.0, 1.0));
    let s = Sphere::new_unit();
    assert!(s.is_intersecting(&r));
    let intersections = s.intersections(&r);
    assert_eq!(s.intersection_count(&r), 2);
    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].t, -6.0);
    assert_eq!(intersections[1].t, -4.0);
}

#[test]
fn test_book_intersection() {
    let s = Sphere::new_unit();
    let r = Ray::new(Tuple::new_point(0.0, 0.0, -5.0), Tuple::new_vector(0.0, 0.0, 1.0));
    let xs = s.intersections(&r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].object, &s);
    assert_eq!(xs[1].object, &s);
}

#[test]
fn test_book_intersection_hits_all_positive() {
    let s = Sphere::new_unit();
    let i1 = Intersection{ t: 1.0, object: &s };
    let i2 = Intersection{ t: 2.0, object: &s };
    let expect = Some(i1.clone());
    let xs = vec![i2, i1];
    let i = Intersection::hit(&xs);
    assert_eq!(i, expect);
}

#[test]
fn test_book_intersection_hits_mixed_positive() {
    let s = Sphere::new_unit();
    let i1 = Intersection{ t: -1.0, object: &s };
    let i2 = Intersection{ t: 1.0, object: &s };
    let expect = Some(i2.clone());
    let xs = vec![i2, i1];
    let i = Intersection::hit(&xs);
    assert_eq!(i, expect);
}

#[test]
fn test_book_intersection_hits_all_negative() {
    let s = Sphere::new_unit();
    let i1 = Intersection{ t: -2.0, object: &s };
    let i2 = Intersection{ t: -1.0, object: &s };
    let xs = vec![i2, i1];
    let i = Intersection::hit(&xs);
    assert_eq!(i, None);
}

#[test]
fn test_book_intersection_is_lowest() {
    let s = Sphere::new_unit();
    let i1 = Intersection{ t: 5.0, object: &s };
    let i2 = Intersection{ t: 7.0, object: &s };
    let i3 = Intersection{ t: -3.0, object: &s };
    let i4 = Intersection{ t: 2.0, object: &s };
    let expect = Some(i4.clone());
    let xs = vec![i1, i2, i3, i4];
    let i = Intersection::hit(&xs);
    assert_eq!(i, expect);
}

#[test]
fn test_book_ray_translation() {
    let r = Ray::new(Tuple::new_point(1.0, 2.0, 3.0), Tuple::new_vector(0.0, 1.0, 0.0));
    let transform = Matrix::translation(3.0, 4.0, 5.0);
    let r2 = transform * r;
    assert_eq!(r2.origin, Tuple::new_point(4.0, 6.0, 8.0));
    assert_eq!(r2.direction, Tuple::new_vector(0.0, 1.0, 0.0));
}
#[test]
fn test_book_ray_scaling() {
    let r = Ray::new(Tuple::new_point(1.0, 2.0, 3.0), Tuple::new_vector(0.0, 1.0, 0.0));
    let transform = Matrix::scaling(2.0, 3.0, 4.0);
    let r2 = transform * r;
    assert_eq!(r2.origin, Tuple::new_point(2.0, 6.0, 12.0));
    assert_eq!(r2.direction, Tuple::new_vector(0.0, 3.0, 0.0));
}

#[test]
fn test_book_intersection_precompute() {
    let r = Ray::new(Tuple::new_point(0.0, 0.0, -5.0), Tuple::new_vector(0.0, 0.0, 1.0));
    let shape = Sphere::new_unit();
    let i = Intersection{ t: 4.0, object: &shape };
    let comps = i.precompute(&r);
    assert_eq!(comps.object, &shape);
    assert_eq!(comps.point, Tuple::new_point(0.0, 0.0, -1.0));
    assert_eq!(comps.eyev, Tuple::new_vector(0.0, 0.0, -1.0));
    assert_eq!(comps.normalv, Tuple::new_vector(0.0, 0.0, -1.0));
    assert_eq!(comps.inside, false);
}

#[test]
fn test_book_intersection_precompute_inside() {
    let r = Ray::new(Tuple::new_point(0.0, 0.0, 0.0), Tuple::new_vector(0.0, 0.0, 1.0));
    let shape = Sphere::new_unit();
    let i = Intersection{ t: 1.0, object: &shape };
    let comps = i.precompute(&r);
    assert_eq!(comps.object, &shape);
    assert_eq!(comps.point, Tuple::new_point(0.0, 0.0, 1.0));
    assert_eq!(comps.eyev, Tuple::new_vector(0.0, 0.0, -1.0));
    assert_eq!(comps.normalv, Tuple::new_vector(0.0, 0.0, -1.0));
    assert_eq!(comps.inside, true);
}