use std::{fmt, ops};

use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3(e0, e1, e2)
    }

    // Computed properties

    pub fn squared_length(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    // Operations

    pub fn dot(&self, other: Vec3) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            0: self.1 * other.2 - self.2 * other.1,
            1: -(self.0 * other.2 - self.2 * other.0),
            2: self.0 * other.1 - self.1 * other.0,
        }
    }
}

// Operator overloading (vector only)

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Invalid index!"),
        }
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Invalid index!"),
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            0: -self.0,
            1: -self.1,
            2: -self.2,
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            0: self.0 + other.0,
            1: self.1 + other.1,
            2: self.2 + other.2,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            0: self.0 + other.0,
            1: self.1 + other.1,
            2: self.2 + other.2,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            0: self.0 - other.0,
            1: self.1 - other.1,
            2: self.2 - other.2,
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            0: self.0 - other.0,
            1: self.1 - other.1,
            2: self.2 - other.2,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            0: self.0 * rhs.0,
            1: self.1 * rhs.1,
            2: self.2 * rhs.2,
        }
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = Vec3 {
            0: self.0 * rhs.0,
            1: self.1 * rhs.1,
            2: self.2 * rhs.2,
        }
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            0: self.0 / rhs.0,
            1: self.1 / rhs.1,
            2: self.2 / rhs.2,
        }
    }
}

impl ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        *self = Vec3 {
            0: self.0 / rhs.0,
            1: self.1 / rhs.1,
            2: self.2 / rhs.2,
        }
    }
}

// Operator overloading (vector and float)

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            0: self.0 * rhs,
            1: self.1 * rhs,
            2: self.2 * rhs,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            0: self * rhs.0,
            1: self * rhs.1,
            2: self * rhs.2,
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Vec3 {
            0: self.0 * rhs,
            1: self.1 * rhs,
            2: self.2 * rhs,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            0: self.0 / rhs,
            1: self.1 / rhs,
            2: self.2 / rhs,
        }
    }
}

impl ops::Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            0: self / rhs.0,
            1: self / rhs.1,
            2: self / rhs.2,
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Vec3 {
            0: self.0 / rhs,
            1: self.1 / rhs,
            2: self.2 / rhs,
        }
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut point = Vec3::new(std::f64::MAX, std::f64::MAX, std::f64::MAX);
    let mut rng = rand::thread_rng();
    loop {
        if point.squared_length() < 1.0 {
            break;
        }
        point = Vec3::new(
            rng.gen_range(-0.5, 0.5),
            rng.gen_range(-0.5, 0.5),
            rng.gen_range(-0.5, 0.5),
        );
    }
    point
}

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
