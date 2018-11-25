#[derive(Debug, Clone, PartialEq)]
pub struct Vector3(pub f32, pub f32, pub f32);

// This trait allows polymorphic functions. It doesn't really scale as each type
// needs to implement every operation. It isn't too different than defining
// different functions for each type eg. `multiply_f32`, and I'm pretty sure
// that is sort of what rust is compiling too anyway.
//
// Another approach is to ensure each type can be cast into a Vector3. Then
// only a single casting function needs to be defined and the operation
// implementations can all be between two vectors.
//
// There isn't much needed here so I'll change this up when I throw some decent
// numbers at it and see what performs best.
pub trait Vector3Operations {
    fn multiple_vector3(&self, other: &Vector3) -> Vector3;
    fn divide_vector3(&self, other: &Vector3) -> Vector3;
}

impl Vector3Operations for Vector3 {
    fn multiple_vector3(&self, other: &Vector3) -> Vector3 {
        Vector3(self.x() * other.x(), self.y() * other.y(), self.z() * other.z())
    }

    fn divide_vector3(&self, other: &Vector3) -> Vector3 {
        Vector3(other.x() / self.x(), other.y() / self.y(), other.z() / self.z())
    }
}

impl Vector3Operations for f32 {
    fn multiple_vector3(&self, other: &Vector3) -> Vector3 {
        Vector3(self * other.x(), self * other.y(), self * other.z())
    }

    fn divide_vector3(&self, other: &Vector3) -> Vector3 {
        Vector3(other.x() / self, other.y() / self, other.z() / self)
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

    pub fn add(&self, other: &Vector3) -> Vector3 {
        Vector3(self.x() + other.x(), self.y() + other.y(), self.z() + other.z())
    }

    pub fn subtract(&self, other: &Vector3) -> Vector3 {
        Vector3(self.x() - other.x(), self.y() - other.y(), self.z() - other.z())
    }

    pub fn multiply<T: Vector3Operations>(&self, other: &T) -> Vector3 {
        other.multiple_vector3(&self)
    }

    pub fn divide<T: Vector3Operations>(&self, other: &T) -> Vector3 {
        other.divide_vector3(&self)
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
        self.divide(&self.magnitude())
    }

    // Linear Interpolation between two vectors at a value, x [0, 1].
    // (1 - x)*start_vec + t*end_vec
    pub fn lerp(&self, end_value: &Vector3, x: f32) -> Vector3 {
        self.multiply(&(1.0 - x)).add(&end_value.multiply(&x))
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
    fn multiple_vector3_vector3() {
        assert_eq!(Vector3(1.0, 4.0, 9.0), test_vec3().multiply(&test_vec3()))
    }

    #[test]
    fn multiple_vector3_f32() {
        assert_eq!(Vector3(2.0, 4.0, 6.0), test_vec3().multiply(&2.0))
    }

    #[test]
    fn divide_vector3_vector3() {
        assert_eq!(Vector3(1.0, 1.0, 1.0), test_vec3().divide(&test_vec3()))
    }

    #[test]
    fn divide_vector3_f32() {
        assert_eq!(Vector3(0.5, 1.0, 1.5), test_vec3().divide(&2.0));
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
