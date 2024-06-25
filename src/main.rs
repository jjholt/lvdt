// mod calibration;

mod plane;
use plane::Plane;
mod plot;

fn main() {
    let plane1 = Plane::new(&[1.0, 1.0, 1.0]);
    let plane2 = Plane::new(&[1.2, 1.0, 1.0]);
    // let plane2 = Plane::new(&[1.4, 1.4, 1.4]);

    // println!("After transform {:#?}", plane2);
    let isometry = plane1.isometry_to(&plane2);
    // println!("Transform 1: {}", isometry.to_matrix());
    // println!("Plane 1:{:#?}", plane1.cartesian_coefficients());
    // println!("Plane 1 After transform {:#?}", plane1.apply_isometry(&transform1).cartesian_coefficients());
    // println!("Plane 2 {:#?}", plane2.cartesian_coefficients());
    // let plane2 = Plane::new(&[10.0, 3.0, 2.5]);
    // let transform2 = plane1.transform_to(&plane2);
    // println!("Transform 2: {}", transform2.to_matrix());

    // Create an arbitrary surface on XY plane:
    let coefficients = Plane::new(&[0.0, 0.0, 0.0])
        .apply_isometry(&isometry)
        .cartesian_coefficients();
    plot::plot("figures/plane.svg", isometry, coefficients)
        .unwrap_or_else(|_| println!("Failed to create image. Check that figures/ folder exists"));
}

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::approx_eq;
    use nalgebra as na;
    #[test]
    fn translation() {
        let plane1 = Plane::new(&[1.0, 1.0, 1.0]);
        let plane2 = Plane::new(&[1.2, 1.2, 1.2]);
        let isometry = plane1.isometry_to(&plane2).to_matrix();
        let mut translation = na::Matrix4::identity();
        translation.m34 = 0.2;
        // This is terrible. it doesn't trace back to the specific structure. float comparison
        // means the normal comparison doesn't work:
        assert_eq!(isometry, translation);
        // isometry.iter().zip(translation.iter()).for_each(|(&a,&b)| {
        //     assert!(approx_eq!(f64, a, b, ulps=2));
        // });
    }
}
