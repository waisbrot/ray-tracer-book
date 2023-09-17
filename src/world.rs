use crate::{light::Light, ray::Ray, tuple::Tuple, color::Color, sphere::Sphere, matrix::Matrix, intersectable::Intersectable, intersection::{Intersection, IntersectionPrecomputation}};

#[derive(Debug)]
pub struct World {
    pub light: Light,
    pub objects: Vec<Box<dyn Intersectable>>,
}

impl World {
    // In book: default_world()
    pub fn default() -> World {
        let mut s1: Sphere = Sphere::new_unit();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = Sphere::new_unit();
        s2.set_transform(Matrix::scaling(0.5, 0.5, 0.5));
        World { 
            light: Light::new_point(Tuple::new_point(-10.0, 10.0, -10.0), Color::white(1.0)), 
            objects: vec![Box::new(s1), Box::new(s2)],
        }
    }

    // In book: intersect_world(world, ray)
    pub fn intersections(&self, ray: &Ray) -> Vec<Intersection> {
        let mut xs: Vec<Intersection> = self.objects.iter().flat_map(|obj| obj.intersections(ray)).collect();
        xs.sort_by(|a,b| a.partial_cmp(b).unwrap());
        xs
    }

    // pg 96
    // In book: shade_hit(world, comps)
    pub fn shade_hit(&self, comps: &IntersectionPrecomputation) -> Color {
        comps.object.material().light(&self.light, &comps.point, &comps.eyev, &comps.normalv)
    }

    // pg 97
    // In book: color_at(w, r)
    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = self.intersections(ray);
        let maybe_hit = Intersection::hit(&intersections);
        match maybe_hit {
            None => 
                Color::white(0.0),
            Some(hit) => {
                let precomp = hit.precompute(ray);
                self.shade_hit(&precomp)
            }
        }
    }
}