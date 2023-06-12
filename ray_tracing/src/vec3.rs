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

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
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
