use nalgebra::{
    self as na, Isometry3, Matrix3, Point2, Point3, Translation3, UnitQuaternion,
    UnitVector3
};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Measurement(pub f64, pub f64, pub f64);

#[derive(Debug)]
pub struct Plane {
    // pub plane: Matrix3<f64>,
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

pub struct PlaneCalibration(Matrix3<f64>);

impl PlaneCalibration {
    pub fn from_vec(points: &[f64; 6]) -> Self {
        let data = points
            .chunks(2)
            .flat_map(|c| vec![c[0], c[1], 0.0])
            .collect();
        PlaneCalibration(Matrix3::from_vec(data))
    }

    pub fn from_points(points: &[Point2<f64>; 3]) -> PlaneCalibration {
        let data = points.iter().flat_map(|c| vec![c.x, c.y, 0.0]).collect();
        PlaneCalibration(Matrix3::from_vec(data))
    }
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
    pub fn as_matrix(&self) -> Matrix3<f64> {
        let v = self.points.iter().flat_map(|f| f.coords.iter().cloned());
        Matrix3::from_iterator(v)
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
    fn normal_from_points(points: &[Point3<f64>]) -> UnitVector3<f64> {
        let v1 = points[2] - points[0];
        let v2 = points[1] - points[0];
        na::Unit::new_normalize(v1.cross(&v2))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creates_calibration_plane() {
        let test = PlaneCalibration::from_vec(&[0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
        PlaneCalibration::from_points(&[
            Point2::new(0.0, 0.0),
            Point2::new(0.0, 10.0),
            Point2::new(10.0, 0.0),
        ]);
    }
}
