use nalgebra::{Isometry3, Point2, Point3, Translation3, UnitQuaternion, UnitVector3};
use serde::de::{self, Deserializer, SeqAccess, Visitor};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Measurement(pub f64, pub f64, pub f64);

impl Measurement {
    pub fn new(measurements: (f64, f64, f64)) -> Measurement {
        let (x, y, z) = measurements;
        Measurement(x, y, z)
    }
}

#[derive(Debug)]
pub struct Plane {
    // pub plane: Matrix3<f64>,
    pub normal: UnitVector3<f64>,
    pub points: Vec<Point3<f64>>,
}
#[allow(unused)]
#[derive(Debug)]
pub struct CartesianCoefficients {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
}

impl Plane {
    // pub fn calibrate(points: [Point2<f64>; 3]) {}

    /// Creates a plane from a vector where the z-cordinate is zero.
    /// The 6 elements of the vector are the x and y coordinates of three points.
    /// my_vec = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    /// Point   |   x   |   y   |       |   x   |   y  |   z  |
    /// 1       |  1.0  |  2.0  |   =>  |  1.0  |  2.0 |  0.0 |
    /// 2       |  3.0  |  4.0  |       |  3.0  |  4.0 |  0.0 |
    /// 3       |  5.0  |  6.0  |       |  5.0  |  6.0 |  0.0 |
    #[allow(unused)]
    pub fn from_vec(points: &[f64; 6]) -> Self {
        let points3: Vec<Point3<f64>> = points
            .chunks(2)
            .map(|c| Point3::new(c[0], c[1], 0.0))
            .collect();
        Self {
            normal: Self::normal_from_points(&points3),
            points: points3,
        }
    }

    /// Creates a plane from a slice of Point2. The resulting Plane has z-coordinate of zero.
    pub fn from_xy(points: &[Point2<f64>; 3]) -> Self {
        let points3: Vec<Point3<f64>> = points.iter().map(|c| Point3::new(c.x, c.y, 0.0)).collect();

        Self {
            normal: Self::normal_from_points(&points3),
            points: points3,
        }
    }
    pub fn new_reading(&self, measurements: &Measurement) -> Plane {
        let mut points = self.points.clone();
        points[0].z = measurements.0;
        points[1].z = measurements.1;
        points[2].z = measurements.2;

        let normal = {
            let v1 = points[2] - points[0];
            let v2 = points[1] - points[0];
            na::Unit::new_normalize(v1.cross(&v2))
        };
        Self { normal, points }
    }

    pub fn isometry_from(&self, other: &Plane) -> Isometry3<f64> {
        let rotation = {
            let axis = self.normal.cross(&other.normal);
            let angle = self.normal.dot(&other.normal).acos();
            UnitQuaternion::from_scaled_axis(axis * angle)
        };

        let translation = Translation3::from(other.points[2] - rotation * self.points[2]);

        Isometry3::from_parts(translation, rotation)
    }

    #[allow(unused)]
    pub fn apply_isometry(&self, isometry: &Isometry3<f64>) -> Plane {
        let normal = isometry * self.normal;
        let points = self.points.iter().map(|p| isometry * p).collect();
        Plane { normal, points }
    }

    #[allow(unused)]
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
    fn normal_from_points(points: &[Point3<f64>]) -> UnitVector3<f64> {
        let v1 = points[2] - points[0];
        let v2 = points[1] - points[0];
        na::Unit::new_normalize(v1.cross(&v2))
    }
}

impl<'de> Deserialize<'de> for Plane {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PlaneVisitor;

        impl<'de> Visitor<'de> for PlaneVisitor {
            type Value = Plane;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence of 6 f64 values")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Plane, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut points = [0.0; 6];
                for i in 0..6 {
                    points[i] = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(i, &self))?;
                }
                Ok(Plane::from_vec(&points))
            }
        }

        deserializer.deserialize_seq(PlaneVisitor)
    }
}
