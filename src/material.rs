use crate::{color::{Color, BLACK}, util::Float, light::Light, tuple::{Point, Vector}};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: Float,
    pub diffuse: Float,
    pub specular: Float,
    pub shininess: Float,
}

impl Material {
    pub fn default() -> Material {
        Material{ color: Color::new(1.0, 1.0, 1.0), ambient: 0.1, diffuse: 0.9, specular: 0.9, shininess: 200.0 }
    }

    pub fn light(&self, light: &Light, position: &Point, eyev: &Vector, normalv: &Vector) -> Color {
        let effective_color = self.color * light.intensity; // combine object and light colors
        let lightv = (light.position - position).normalize().unwrap();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = lightv.dot(normalv);
        let mut diffuse = BLACK.clone();
        let mut specular = BLACK.clone();
        if light_dot_normal >= 0.0 {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = (-lightv).reflect(normalv).unwrap();
            let reflect_dot_eye = reflectv.dot(eyev);
            if reflect_dot_eye > 0.0 {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }
        ambient + diffuse + specular
    }
}