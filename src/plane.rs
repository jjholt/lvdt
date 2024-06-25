use nalgebra::{self as na, Isometry3, Matrix3x4, Point3, Translation3, UnitQuaternion, UnitVector3, Vector3};

#[derive(Debug)]
pub struct Plane {
    pub normal: UnitVector3<f64>,
    pub points: Vec<Point3<f64>>,
}
#[derive(Debug)]
pub struct CartesianCoefficients {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
}

impl Plane {
    pub fn new(measurements: &[f64; 3]) -> Plane {
        let points = vec![
            Point3::new(0.0, 0.0, measurements[0]),
            Point3::new(0.0, 10.0, measurements[1]),
            Point3::new(10.0, 0.0, measurements[2]),
        ];

        let normal = {
            let v1 = points[2] - points[0];
            let v2 = points[1] - points[0];
            na::Unit::new_normalize(v1.cross(&v2))
        };
        Self { normal, points }
    }

    pub fn isometry_to(&self, other: &Plane) -> Isometry3<f64> {
        let rotation = {
            let axis = self.normal.cross(&other.normal);
            let angle = self.normal.dot(&other.normal).acos();
            UnitQuaternion::from_scaled_axis(axis * angle)
        };

        let translation = Translation3::from(other.points[2] - rotation * self.points[2]);

        Isometry3::from_parts(translation, rotation)
    }

    pub fn apply_isometry(&self, isometry: &Isometry3<f64>) -> Plane {
        let normal = isometry * self.normal;
        let points = self.points.iter().map(|p| isometry * p).collect();
        Plane { normal, points }
    }

    pub fn cartesian_coefficients(&self) -> CartesianCoefficients {
        let d = self.points[0] // Use iterator here. Structure was changed from array to vector
            .iter()
            .zip(self.normal.iter())
            .map(|(p, n)| p * n)
            .sum();
        CartesianCoefficients {
            a: self.normal.x,
            b: self.normal.y,
            c: self.normal.z,
            d,
        }
    }
}
