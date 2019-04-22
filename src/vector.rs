use std::ops;

use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// Construction
impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }
}

// Computed properties
impl Vector {
    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn unit_vector(&self) -> Vector {
        *self / self.length()
    }
}

// Vector-specific operations
impl Vector {
    pub fn dot(lhs: &Vector, rhs: &Vector) -> f64 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn cross(lhs: &Vector, rhs: &Vector) -> Vector {
        Vector {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: -(lhs.x * rhs.z - lhs.z * rhs.x),
            z: lhs.x * rhs.y - lhs.y * rhs.x,
        }
    }

    /// Generate random Vector within a unit sphere.
    pub fn random() -> Vector {
        let mut point = Vector::new(std::f64::MAX, std::f64::MAX, std::f64::MAX);
        let mut rng = rand::thread_rng();
        loop {
            if point.squared_length() < 1.0 {
                break;
            }
            point = Vector::new(
                rng.gen_range(-0.5, 0.5),
                rng.gen_range(-0.5, 0.5),
                rng.gen_range(-0.5, 0.5),
            );
        }
        point
    }
}

// Operator overloading (vector only)

impl ops::Index<usize> for Vector {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index!"),
        }
    }
}

impl ops::IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid index!"),
        }
    }
}

impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        *self = Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::SubAssign for Vector {
    fn sub_assign(&mut self, other: Vector) {
        *self = Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::MulAssign<Vector> for Vector {
    fn mul_assign(&mut self, rhs: Vector) {
        *self = Vector {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Div<Vector> for Vector {
    type Output = Vector;

    fn div(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl ops::DivAssign<Vector> for Vector {
    fn div_assign(&mut self, rhs: Vector) {
        *self = Vector {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

// Operator overloading (vector and float)

impl ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Vector {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Vector {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Div<Vector> for f64 {
    type Output = Vector;

    fn div(self, rhs: Vector) -> Vector {
        Vector {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}

impl ops::DivAssign<f64> for Vector {
    fn div_assign(&mut self, rhs: f64) {
        *self = Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    // Utility function to check the approximate equality of two vectors.
    //
    // Direct equality comparison between two vectors is unavailable due to
    // the underlying float representation.
    fn vec_approx_equal(v1: Vector, v2: Vector) {
        assert_approx_eq!(v1.x, v2.x);
        assert_approx_eq!(v1.y, v2.y);
        assert_approx_eq!(v1.z, v2.z);
    }

    // Construction

    #[test]
    fn vector_new() {
        let given = Vector::new(1.0, 2.0, 3.0);
        let expected = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        vec_approx_equal(given, expected);
    }

    // Computed properties

    #[test]
    fn vector_squared_length() {
        let given = Vector::new(0.0, 0.0, 0.0).squared_length();
        let expected = 0.0;
        assert_approx_eq!(given, expected);

        let given = Vector::new(1.0, 2.0, 3.0).squared_length();
        let expected = 14.0;
        assert_approx_eq!(given, expected);
    }

    #[test]
    fn vector_length() {
        let given = Vector::new(0.0, 0.0, 0.0).length();
        let expected = 0.0;
        assert_approx_eq!(given, expected);

        let given = Vector::new(1.0, 2.0, 3.0).length();
        let expected = 14.0_f64.sqrt();
        assert_approx_eq!(given, expected);
    }

    #[test]
    fn vector_unit_vector() {
        let given = Vector::new(1.0, 2.0, 3.0).unit_vector();
        let expected = Vector::new(0.267261, 0.534522, 0.801784);
        vec_approx_equal(given, expected);
    }

    // Vector operations

    #[test]
    fn vector_dot() {
        let given = Vector::dot(&Vector::new(1.0, 2.0, 3.0), &Vector::new(4.0, 5.0, 6.0));
        let expected = 32.0;
        assert_approx_eq!(given, expected);
    }

    #[test]
    fn vector_cross() {
        let given = Vector::cross(&Vector::new(1.0, 2.0, 3.0), &Vector::new(4.0, 5.0, 6.0));
        let expected = Vector::new(-3.0, 6.0, -3.0);
        vec_approx_equal(given, expected);
    }

    // Operator overloading (vector only)

    #[test]
    fn vector_index() {
        let given = Vector::new(1.0, 2.0, 3.0);
        assert_approx_eq!(given[0], 1.0);
        assert_approx_eq!(given[1], 2.0);
        assert_approx_eq!(given[2], 3.0);
    }

    #[test]
    #[should_panic]
    fn vector_index_out_of_bounds() {
        let given = Vector::new(1.0, 2.0, 3.0);
        let _ = given[4];
    }

    #[test]
    fn vector_index_mut() {
        let mut given = Vector::new(1.0, 2.0, 3.0);
        given[0] = 2.0;
        given[1] = 3.0;
        given[2] = 4.0;
        let expected = Vector::new(2.0, 3.0, 4.0);
        vec_approx_equal(given, expected);
    }

    #[test]
    #[should_panic]
    fn vector_index_mut_out_of_bounds() {
        let mut given = Vector::new(1.0, 2.0, 3.0);
        given[4] = 4.0;
    }

    #[test]
    fn vector_neg() {
        let given = -Vector::new(-1.0, 2.0, -3.0);
        let expected = Vector::new(1.0, -2.0, 3.0);
        vec_approx_equal(given, expected);
    }

    #[test]
    fn vector_add() {
        let given = Vector::new(1.0, 2.0, 3.0) + Vector::new(3.0, 2.0, 1.0);
        let expected = Vector::new(4.0, 4.0, 4.0);
        vec_approx_equal(given, expected);
    }

    #[test]
    fn vector_add_assign() {
        let mut given = Vector::new(1.0, 2.0, 3.0);
        given += Vector::new(3.0, 2.0, 1.0);
        let expected = Vector::new(4.0, 4.0, 4.0);
        vec_approx_equal(given, expected);
    }

    #[test]
    fn vector_sub() {
        let given = Vector::new(1.0, 2.0, 3.0) - Vector::new(3.0, 2.0, 1.0);
        let expected = Vector::new(-2.0, 0.0, 2.0);
        vec_approx_equal(given, expected);
    }

    #[test]
    fn vector_sub_assign() {
        let mut given = Vector::new(1.0, 2.0, 3.0);
        given -= Vector::new(3.0, 2.0, 1.0);
        let expected = Vector::new(-2.0, 0.0, 2.0);
        vec_approx_equal(given, expected);
    }

    #[test]
    fn vector_mul_vector() {
        let given = Vector::new(1.0, 2.0, 3.0) * Vector::new(3.0, 2.0, 1.0);
        let expected = Vector::new(3.0, 4.0, 3.0);
        vec_approx_equal(given, expected);
    }

    #[test]
    fn vector_mul_assign_vector() {
        let mut given = Vector::new(1.0, 2.0, 3.0);
        given *= Vector::new(3.0, 2.0, 1.0);
        let expected = Vector::new(3.0, 4.0, 3.0);
        vec_approx_equal(given, expected);
    }

    #[test]
    fn vector_div_vector() {
        let given = Vector::new(1.0, 2.0, 3.0) / Vector::new(3.0, 2.0, 1.0);
        let expected = Vector::new(0.333333, 1.0, 3.0);
        vec_approx_equal(given, expected);
    }

    #[test]
    fn vector_div_assign_vector() {
        let mut given = Vector::new(1.0, 2.0, 3.0);
        given /= Vector::new(3.0, 2.0, 1.0);
        let expected = Vector::new(0.333333, 1.0, 3.0);
        vec_approx_equal(given, expected);
    }

    // Operator overloading (vector only)

    #[test]
    fn vector_mul_float() {
        let given = Vector::new(1.0, 2.0, 3.0) * 3.0;
        let expected = Vector::new(3.0, 6.0, 9.0);
        vec_approx_equal(given, expected);
    }

    #[test]
    fn float_mul_vector() {
        let given = 3.0 * Vector::new(1.0, 2.0, 3.0);
        let expected = Vector::new(3.0, 6.0, 9.0);
        vec_approx_equal(given, expected);
    }

    #[test]
    fn vector_mul_assign_float() {
        let mut given = Vector::new(1.0, 2.0, 3.0);
        given *= 3.0;
        let expected = Vector::new(3.0, 6.0, 9.0);
        vec_approx_equal(given, expected);
    }

    #[test]
    fn vector_div_float() {
        let given = Vector::new(1.0, 2.0, 3.0) / 2.0;
        let expected = Vector::new(0.5, 1.0, 1.5);
        vec_approx_equal(given, expected);
    }

    #[test]
    fn float_div_vector() {
        let given = 2.0 / Vector::new(1.0, 2.0, 3.0);
        let expected = Vector::new(2.0, 1.0, 0.666667);
        vec_approx_equal(given, expected);
    }

    #[test]
    fn vector_div_assign_float() {
        let mut given = Vector::new(1.0, 2.0, 3.0);
        given /= 2.0;
        let expected = Vector::new(0.5, 1.0, 1.5);
        vec_approx_equal(given, expected);
    }
}
