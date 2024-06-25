use nalgebra::{Isometry3, Matrix3, Matrix4, Point3, Vector4};
use plotters::prelude::*;

use crate::plane::{CartesianCoefficients, Plane};

pub fn plot(
    filename: &str,
    isometry: Isometry3<f64>,
    coefficients: CartesianCoefficients,
) -> Result<(), Box<dyn std::error::Error>> {
    let area = SVGBackend::new(filename, (1024, 760)).into_drawing_area();

    area.fill(&WHITE)?;

    let x_axis = (-2.0..2.0).step(0.1);
    let z_axis = (-2.0..2.0).step(0.1);

    let mut chart = ChartBuilder::on(&area)
        .caption("Transformation of arbitrary plane", ("Sans", 30))
        .set_left_and_bottom_label_area_size(40)
        .build_cartesian_3d(x_axis, 0.0..0.5, z_axis)?;

    chart.with_projection(|mut pb| {
        pb.yaw = 0.3;
        pb.pitch = 0.3;
        pb.scale = 0.9;
        pb.into_matrix()
    });

    chart
        .configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(3)
        .draw()?;

    let xy = (-2..2).map(|f| f as f64);
    chart.draw_series(
        SurfaceSeries::xoz(xy.clone(), xy.clone(), |_, _| 0.0).style(&BLUE.mix(0.2)),
    )?;

    let transformed_xy = xy
        .clone()
        .map(|f| isometry.transform_point(&Point3::new(f, f, 0.0)));
    let trans_x = transformed_xy.clone().map(|p| p.x);
    let trans_y = transformed_xy.clone().map(|p| p.y);

    let a = coefficients.a;
    let b = coefficients.b;
    let c = coefficients.c;
    let d = coefficients.d;

    chart.draw_series(
        SurfaceSeries::xoz(trans_x, trans_y, |x, y| 1.0 / c * (a * x + b * y + d))
            .style(&RED.mix(0.2)),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", filename);
    Ok(())
}
