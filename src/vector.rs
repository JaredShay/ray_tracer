#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Vector3(pub f32, pub f32, pub f32);

pub trait IntoVector3 {
    fn into_vector3(&self) -> Vector3;
}

impl IntoVector3 for Vector3 {
    fn into_vector3(&self) -> Vector3 {
        self.clone()
    }
}

impl IntoVector3 for f32 {
    fn into_vector3(&self) -> Vector3 {
        Vector3(*self, *self, *self)
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
    println!("v1 * v2: {:?}", vector_1.multiply(vector_2));
    }

    pub fn subtract(&self, other: &Vector3) -> Vector3 {
        Vector3(self.x() - other.x(), self.y() - other.y(), self.z() - other.z())
    }

    pub fn multiply<T: IntoVector3>(&self, other: &T) -> Vector3 {
        let _other = other.into_vector3();

        Vector3(self.x() * _other.x(), self.y() * _other.y(), self.z() * _other.z())
    }

    pub fn divide<T: IntoVector3>(&self, other: &T) -> Vector3 {
        let _other = other.into_vector3();

        Vector3(self.x() / _other.x(), self.y() / _other.y(), self.z() / _other.z())
    }

    pub fn dot_product(&self, other: &Vector3) -> f32 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }
}
