use nalgebra::{Isometry3, UnitQuaternion, Vector3};

use calibration::LVDT;
mod calibration;

fn main() {
    let lvdt = LVDT::new();
    println!("Translations: {}", lvdt.translations());
    println!("Rotations: {}", lvdt.rotations());
}
