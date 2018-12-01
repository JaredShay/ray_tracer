use std::ops;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector3(pub f32, pub f32, pub f32);

// Vector3 + &Vector3
impl<'a> ops::Add<&'a Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &'a Vector3) -> Vector3 {
        Vector3(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

// &Vector3 + &Vector3
impl<'a, 'b> ops::Add<&'b Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &'b Vector3) -> Vector3 {
        Vector3(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

// Vector3 * &Vector3
impl<'a> ops::Mul<&'a Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: &'a Vector3) -> Vector3 {
        Vector3(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

// &Vector3 * f32
impl<'a> ops::Mul<f32> for &'a Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Vector3 {
        Vector3(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

// &Vector3 / f32
impl<'a> ops::Div<f32> for &'a Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Vector3 {
        Vector3(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

#[allow(dead_code)]
impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3(x, y, z)
    }

    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn z(&self) -> f32 {
        self.2
    }

    pub fn dot_product(&self, other: &Vector3) -> f32 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross_product(&self, other: &Vector3) -> Vector3 {
        Vector3(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x())
    }

    pub fn unit_vector(&self) -> Vector3 {
        self / self.magnitude()
    }

    // Linear Interpolation between two vectors at a value, x [0, 1].
    pub fn lerp(&self, end_value: &Vector3, x: f32) -> Vector3 {
        self * (1.0 - x) + &(end_value * x)
    }

    fn magnitude(&self) -> f32 {
        (self.x().powi(2) + self.y().powi(2) + self.z().powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use vector::Vector3;

    fn test_vec3() -> Vector3 {
        Vector3(1.0, 2.0, 3.0)
    }

    #[test]
    fn x() {
        assert_eq!(1.0, test_vec3().x())
    }

    #[test]
    fn y() {
        assert_eq!(2.0, test_vec3().y())
    }

    #[test]
    fn z() {
        assert_eq!(3.0, test_vec3().z())
    }

    #[test]
    fn add() {
        assert_eq!(Vector3(2.0, 4.0, 6.0), test_vec3().add(&test_vec3()))
    }

    #[test]
    fn subtract() {
        assert_eq!(Vector3(0.0, 0.0, 0.0), test_vec3().subtract(&test_vec3()))
    }

    #[test]
    fn multiple_vector3_ref_vector3() {
        assert_eq!(Vector3(1.0, 4.0, 9.0), test_vec3() * &test_vec3())
    }

    #[test]
    fn multiple_vector3_f32() {
        assert_eq!(Vector3(2.0, 4.0, 6.0), test_vec3() * 2.0)
    }


    #[test]
    fn dot_product_perpendicular() {
        let vector3_1 = Vector3(0.0, 1.0, 0.0);
        let vector3_2 = Vector3(1.0, 0.0, 0.0);

        assert_eq!(0.0, vector3_1.dot_product(&vector3_2));
    }

    #[test]
    fn dot_product_self() {
        assert_eq!(14.0, test_vec3().dot_product(&test_vec3()));
    }

    #[test]
    fn cross_product() {
        let vector3_1 = Vector3(0.0, 1.0, 0.0);
        let vector3_2 = Vector3(1.0, 0.0, 0.0);

        assert_eq!(Vector3(0.0, 0.0, -1.0), vector3_1.cross_product(&vector3_2));
        assert_eq!(Vector3(0.0, 0.0, 1.0), vector3_2.cross_product(&vector3_1))
    }

    #[test]
    fn unit_vector() {
        let vector3 = Vector3(1.0, 0.0, 0.0);

        assert_eq!(Vector3(1.0, 0.0, 0.0), vector3.unit_vector());
    }
}
