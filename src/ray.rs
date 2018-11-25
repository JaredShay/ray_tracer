#[allow(dead_code)]

use vector::{Vector3, Vector3Operations};

#[derive(Debug)]
pub struct Ray<'a> {
    pub origin: &'a Vector3,
    pub direction: Vector3
}

impl<'a> Ray<'a> {
    pub fn point_at_parameter(&self, parameter: f32) -> Vector3 {
        self.origin.add(&self.direction.multiply(&parameter))
    }
}

#[cfg(test)]
mod tests {
    use ray::Ray;
    use vector::{Vector3, Vector3Operations};

    #[test]
    fn point_at_parameter() {
        let ray = Ray {
            origin: Vector3(0.0, 0.0, 0.0),
            direction: Vector3(1.0, 2.0, 0.0),
        };

        assert_eq!(Vector3(0.5, 1.0, 0.0), ray.point_at_parameter(0.5))
    }
}
