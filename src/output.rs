use na::Isometry3;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Output {
    x: f64,
    y: f64,
    z: f64,
    transverse: f64,
    sagittal: f64,
    coronal: f64,
}

impl Output {
    pub fn new(isometry: &Isometry3<f64>) -> Output {
        let translation = isometry.translation;
        let rotation = isometry.rotation.euler_angles();

        Output {
            x: translation.x,
            y: translation.y,
            z: translation.z,
            coronal: rotation.0,
            sagittal: rotation.1,
            transverse: rotation.2,
        }
    }
}
