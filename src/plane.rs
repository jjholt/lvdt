use nalgebra::{
    self as na, Isometry3, Matrix4, Point3, Translation3, UnitQuaternion, UnitVector3
};

#[derive(Debug)]
pub struct Plane {
    pub normal: UnitVector3<f64>,
    pub points: [Point3<f64>;3]
}

impl Plane {
    pub fn new(measurements: &[f64; 3]) -> Plane {
        let points = [
            Point3::new(0.0, 0.0, measurements[0]),
            Point3::new(0.0, 1.0, measurements[1]),
            Point3::new(1.0, 0.0, measurements[2]),
        ];

        let normal = {
            let v1 = points[1] - points[0];
            let v2 = points[2] - points[0];
            na::Unit::new_normalize(v1.cross(&v2))
        };

        Self {
            normal,
            points
        }
    }

    pub fn transform_to(&self, other: &Plane) -> Matrix4<f64> {
        let rotation = {
            let axis = na::Unit::new_normalize(self.normal.cross(&other.normal));
            let angle = self.normal.dot(&other.normal);
            UnitQuaternion::from_axis_angle(&axis, angle)
        };
        let translation = Translation3::from(other.points[0] - rotation*self.points[0]);

        Isometry3::from_parts(translation, rotation).to_matrix()
    }
}
