use crate::{util::Float, tuple::{Point, Vector}, intersectable::Intersectable, ray::Ray};

#[derive(Debug, Clone)]
pub struct IntersectionPrecomputation<'a> {
    pub t: Float,
    pub object: &'a dyn Intersectable,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
}

// In book: intersection(int, object)
#[derive(Debug, Clone)]
pub struct Intersection<'a> {
    pub t: Float,
    pub object:  &'a dyn Intersectable,
}

impl <'a> Intersection<'a> {
    // In book: hit(xs)
    pub fn hit(intersections: &Vec<Intersection<'a>>) -> Option<Intersection<'a>> {
        intersections.iter()
        .filter(|i| i.t >= 0.0)
        .reduce(|acc, e| if e < acc { e } else { acc })
        .map(|i| i.clone())
    }

    // In book: prepare_computations(int, ray)
    pub fn precompute(&self, ray: &Ray) -> IntersectionPrecomputation {
        let t = self.t;
        let object = self.object;
        let point = ray.position(&self.t);
        let eyev = -ray.direction;
        let normalv = self.object.surface_normal(&point);
        let inside = normalv.dot(&eyev) < 0.0;
        let normalv = if inside {
            -normalv
        } else {
            normalv
        };
        IntersectionPrecomputation { t, object, point, eyev, normalv, inside }
    }
}

impl PartialOrd for Intersection<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

impl PartialEq<Intersection<'_>> for Intersection<'_> {
    fn eq(&self, other: &Intersection<'_>) -> bool {
        self.t == other.t && self.object as *const dyn Intersectable as *const () == other.object as *const dyn Intersectable as *const ()
    }
}
impl PartialEq<Intersection<'_>> for &Intersection<'_> {
    fn eq(&self, other: &Intersection<'_>) -> bool {
        self.t == other.t && self.object as *const dyn Intersectable as *const () == other.object as *const dyn Intersectable as *const ()
    }
}
impl PartialEq<&Intersection<'_>> for Intersection<'_> {
    fn eq(&self, other: &&Intersection) -> bool {
        self.t == other.t && self.object as *const dyn Intersectable as *const () == other.object as *const dyn Intersectable as *const ()
    }
}
