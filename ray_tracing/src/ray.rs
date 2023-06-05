use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
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

        assert_eq!(r.origin().x(), 1.0);
        assert_eq!(r.origin().y(), 2.0);
        assert_eq!(r.origin().z(), 0.0);
    }

    #[test]
    fn test_ray_direction() {
        let r = Ray::new(Vec3::new(1.0, 2.0, 0.0), Vec3::new(1.0, -2.0, 5.0));

        assert_eq!(r.direction().x(), 1.0);
        assert_eq!(r.direction().y(), -2.0);
        assert_eq!(r.direction().z(), 5.0);
    }

    #[test]
    fn test_ray_at() {
        let r = Ray::new(Vec3::new(1.0, 2.0, 0.0), Vec3::new(1.0, -2.0, 5.0));

        assert_eq!(r.at(5.0).x(), 6.0);
        assert_eq!(r.at(5.0).y(), -8.0);
        assert_eq!(r.at(5.0).z(), 25.0);
    }
}
