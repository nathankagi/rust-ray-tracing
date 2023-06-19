use rand::Rng;
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn dot(u: Vec3, v: Vec3) -> f64 {
        u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
    }

    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3::new(
            u.y() * v.x() - u.z() * v.y(),
            u.z() * v.x() - u.x() * v.z(),
            u.x() * v.y() - u.y() * v.x(),
        )
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - (n * 2.0 * Vec3::dot(v, n))
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = Vec3::dot(-uv, n);
        let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
        let r_out_parallel = n * -((1.0 - r_out_perp.length_squared()) as f64).abs().sqrt();
        r_out_perp + r_out_parallel
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            rand::thread_rng().gen_range(min..max),
            rand::thread_rng().gen_range(min..max),
            rand::thread_rng().gen_range(min..max),
        )
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::unit_vector(Vec3::random_in_unit_sphere())
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                return p;
            }
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1.0e-8;
        return self.x().abs() < s && self.y().abs() < s && self.z().abs() < s;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        )
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: f64) -> Vec3 {
        Vec3::new(self.x() * v, self.y() * v, self.z() * v)
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x() / other.x(),
            self.y() / other.y(),
            self.z() / other.z(),
        )
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, v: f64) -> Vec3 {
        Vec3::new(self.x() / v, self.y() / v, self.z() / v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vec3() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_vec3_operators() {
        let v = Vec3::new(1.0, 2.0, 3.0) + Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(v, Vec3::new(3.0, 4.0, 5.0));

        let v = Vec3::new(1.0, 2.0, 3.0) - Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(v, Vec3::new(-1.0, 0.0, 1.0));

        let v = Vec3::new(1.0, 2.0, 3.0) * Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(v, Vec3::new(2.0, 4.0, 6.0));

        let v = Vec3::new(1.0, 2.0, 3.0) / Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(v, Vec3::new(0.5, 1.0, 1.5));

        let v = Vec3::new(2.0, 3.0, 6.0) / 4.0;
        assert_eq!(v, Vec3::new(0.5, 3.0 / 4.0, 6.0 / 4.0));
    }
}
