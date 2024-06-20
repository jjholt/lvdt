use na::{Matrix6, MatrixView3, UnitQuaternion};
use nalgebra as na;

#[allow(clippy::upper_case_acronyms)]
pub struct LVDT {
    /// The matrix that represents six degrees of freedom [x,y,z, Rx, Ry, Rz] is, in order,
    /// [medial-lateral, proximal-distal, anterior-posterior, flexion-extension, internal-external,
    /// varus-valgus]
    pub matrix: Matrix6<f64>,
}

impl LVDT {
    pub fn new() -> LVDT {
        Self {
            matrix: Matrix6::identity(),
        }
    }
    pub fn translations(&self) -> MatrixView3<f64, na::Const<1>, na::Const<6>> {
        self.matrix.fixed_view::<3, 3>(0, 0)
    }
    pub fn rotations(&self) -> MatrixView3<f64, na::Const<1>, na::Const<6>> {
        self.matrix.fixed_view::<3, 3>(3, 3)
    }
    /// Returns a view into the matrix that corresponds to the medial-lateral axis.
    pub fn x(&self) {
        self.medial_lateral()
    }
    /// Rotation about the x-axis (medial-lateral). Returns flexion-extension rotation.
    pub fn rx(&self) {
        self.flexion_extension()
    }
    /// Returns a view into the matrix that corresponds to the proximal-distal axis.
    pub fn y(&self) {
        self.proximal_distal()
    }
    /// Rotation about the y-axis (proximal-distal). Returns internal-external rotation.
    pub fn ry(&self) {
        self.internal_external()
    }
    /// Returns a view into the matrix that corresponds to the anterior-posterior axis.
    pub fn z(&self) {
        self.anterior_posterior()
    }
    /// Rotation about the z-axis (anterior-posterior). Returns varus-valgus rotation.
    pub fn rz(&self) {
        self.varus_valgus()
    }
    pub fn varus_valgus(&self) {
        todo!();
    }
    pub fn flexion_extension(&self) {
        todo!();
    }
    pub fn internal_external(&self) {
        todo!();
    }

    pub fn anterior_posterior(&self) {
        todo!();
    }
    pub fn medial_lateral(&self) {
        todo!();
    }
    pub fn proximal_distal(&self) {
        todo!();
    }
}
