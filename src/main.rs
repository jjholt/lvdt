// mod calibration;

mod plane;
use plane::Plane;
mod plot;

fn main() {
    let plane1 = Plane::new(&[5.0, 1.0, 1.0]);
    let plane2 = Plane::new(&[6.0, 1.0, 1.0]);
    let transform1 = plane1.transform_to(&plane2);
    // println!("Transform 1: {}", transform1.to_matrix());
    // let plane2 = Plane::new(&[10.0, 3.0, 2.5]);
    // let transform2 = plane1.transform_to(&plane2);
    // println!("Transform 2: {}", transform2.to_matrix());

    plot::plot("figures/plane.svg", transform1).unwrap_or_else(|_| println!("Failed to print"));
}
