use na::{Matrix6, MatrixView3};
use nalgebra as na;

#[allow(clippy::upper_case_acronyms)]
pub struct LVDT {
    matrix: Matrix6<f64>,
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
        self.matrix.fixed_view::<3, 3>(3, 0)
    }
    pub fn x(&self) {
        todo!();
    }
    pub fn y(&self) {
        todo!();
    }
    pub fn z(&self) {
        todo!();
    }
    pub fn ap(&self) {
        todo!();
    }
    pub fn ml(&self) {
        todo!();
    }
    pub fn ie(&self) {
        todo!();
    }
}
