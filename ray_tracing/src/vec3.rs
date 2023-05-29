use std::io;
use std::ops;

pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn dot(u: Vec3, v: Vec3) -> f64 {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }

    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3 {
            e: [
                u.e[1] * v.e[0] - u.e[2] * v.e[1],
                u.e[2] * v.e[0] - u.e[0] * v.e[2],
                u.e[0] * v.e[1] - u.e[1] * v.e[0],
            ],
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        let len = self.length();
        Vec3 {
            e: [self.e[0] / len, self.e[1] / len, self.e[2] / len],
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vec3 {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Vec3 {
            e: [
                self.e[0] / other.e[0],
                self.e[1] / other.e[1],
                self.e[2] / other.e[2],
            ],
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, v: f64) -> Self {
        Vec3 {
            e: [self.e[0] / v, self.e[1] / v, self.e[2] / v],
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_create_vec3() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.e[0], 1.0);
        assert_eq!(v.e[1], 2.0);
        assert_eq!(v.e[2], 3.0);
    }

    #[test]
    fn test_vec3_operators() {
        let v = Vec3::new(1.0, 2.0, 3.0) + Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(v.e[0], 3.0);
        assert_eq!(v.e[1], 4.0);
        assert_eq!(v.e[2], 5.0);

        let v = Vec3::new(1.0, 2.0, 3.0) - Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(v.e[0], -1.0);
        assert_eq!(v.e[1], 0.0);
        assert_eq!(v.e[2], 1.0);

        let v = Vec3::new(1.0, 2.0, 3.0) * Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(v.e[0], 2.0);
        assert_eq!(v.e[1], 4.0);
        assert_eq!(v.e[2], 6.0);

        let v = Vec3::new(1.0, 2.0, 3.0) / Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(v.e[0], 0.5);
        assert_eq!(v.e[1], 1.0);
        assert_eq!(v.e[2], 1.5);
    }
}
