use crate::structures::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin() + self.direction() * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_orig() {
        let r = Ray::new(Vec3::new(1.0, 2.0, 0.0), Vec3::new(1.0, -2.0, 5.0));
        assert_eq!(r.origin(), Vec3::new(1.0, 2.0, 0.0));
    }

    #[test]
    fn test_ray_direction() {
        let r = Ray::new(Vec3::new(1.0, 2.0, 0.0), Vec3::new(1.0, -2.0, 5.0));
        assert_eq!(r.direction(), Vec3::new(1.0, -2.0, 5.0));
    }

    #[test]
    fn test_ray_at() {
        let r = Ray::new(Vec3::new(1.0, 2.0, 0.0), Vec3::new(1.0, -2.0, 5.0));
        assert_eq!(r.at(5.0), Vec3::new(6.0, -8.0, 25.0));
    }
}
