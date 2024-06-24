// mod calibration;

mod plane;
use plane::Plane;

fn main() {
    let plane1 = Plane::new(&[5.0, 3.0, 2.5]);
    let plane2 = Plane::new(&[6.0, 3.0, 2.5]);
    println!("{}", plane1.transform_to(&plane2));
    let plane2 = Plane::new(&[10.0, 3.0, 2.5]);
    println!("{}", plane1.transform_to(&plane2));
}
