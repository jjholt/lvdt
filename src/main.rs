use calibration::LVDT;

mod calibration;
fn main() {

    let lvdt = LVDT::new();
    println!("{}", lvdt.translations());
    println!("{}", lvdt.rotations());
}
