use crate::vector::Vector;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn point_at_parameter(&self, t: f64) -> Vector {
        self.origin + t * self.direction
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
    fn ray_new() {
        let given = Ray::new(Vector::new(0.0, 1.0, 2.0), Vector::new(2.0, 1.0, 0.0));
        let expected = Ray {
            origin: Vector::new(0.0, 1.0, 2.0),
            direction: Vector::new(2.0, 1.0, 0.0),
        };
        vec_approx_equal(given.origin, expected.origin);
        vec_approx_equal(given.direction, expected.direction);
    }

    // Methods

    #[test]
    fn ray_point_at_parameter() {
        let given = Ray::new(Vector::new(0.0, 1.0, 2.0), Vector::new(1.0, 1.0, 1.0))
            .point_at_parameter(0.5);
        let expected = Vector::new(0.5, 1.5, 2.5);
        vec_approx_equal(given, expected);
    }

}
