#![feature(iterator_try_collect)]
extern crate approx;
extern crate nalgebra as na;
mod plane;
use std::{
    fs,
    io::{self, IsTerminal},
    path::PathBuf,
};

use clap::Parser;
use output::Output;
use plane::{Measurement, Plane};
use serde::Deserialize;
mod plot;

mod output;

#[derive(Debug, Parser)]
struct Args {
    /// Calibration data
    #[arg(short,long, default_value_t = String::from("calibration.csv"))]
    calibration: String,
    /// Data
    #[arg(short,long, default_value_t = String::from("data.csv"))]
    data: String,
}

#[derive(Deserialize)]
struct Config {
    screws: Plane,
    implant: Plane,
}

fn main() {
    let args = Args::parse();
    let calibration = deserialise(args.calibration.into()).unwrap();

    let yaml = fs::read_to_string("config.yaml").expect("Missing config.yaml file in root");
    let config: Config = serde_yaml::from_str(&yaml).unwrap();
    let base_plane = config.screws;
    let implant = config.implant;

    let screw = base_plane.new_reading(&average(&calibration));

    let mut wtr = csv::Writer::from_writer(io::stdout());

    let input = new_input((&args.data).into());
    match input {
        Ok(measurement) => measurement
            .iter()
            .map(|m| screw.new_reading(m))
            .map(|p| screw.isometry_from(&p))
            .map(|i| implant.apply_isometry(&i))
            .map(|p| implant.isometry_from(&p))
            .map(|f| Output::new(&f))
            .for_each(|f| wtr.serialize(f).unwrap()),
        Err(err) => {
            eprintln!("Unable to open input. {}: {}", &args.data, err);
            std::process::exit(1)
        }
    }

    wtr.flush().unwrap();

    // // Create an arbitrary surface on XY plane:
    // let coefficients = base_plane
    //     .new_reading(&Measurement(0.0, 0.0, 0.0))
    //     .apply_isometry(&isometry1)
    //     .cartesian_coefficients();
    // plot::plot("figures/translation.svg", isometry1, coefficients).unwrap();
    //
    // let coefficients = base_plane
    //     .new_reading(&Measurement(0.0, 0.0, 0.0))
    //     .apply_isometry(&isometry2)
    //     .cartesian_coefficients();
    // plot::plot("figures/rotation.svg", isometry2, coefficients).unwrap();
}

fn average(measurements: &[Measurement]) -> Measurement {
    let len = measurements.len() as f64;
    let v = measurements.iter().fold((0.0, 0.0, 0.0), |acc, m| {
        (acc.0 + m.0 / len, acc.1 + m.1 / len, acc.2 + m.2 / len)
    });
    Measurement(v.0, v.1, v.2)
}

fn deserialise<'a>(path: PathBuf) -> Result<Vec<Measurement>, csv::Error> {
    let mut rdr = csv::Reader::from_path(&path).expect(&format!(
        "Unable to open path: '{}'",
        &path.to_str().unwrap_or("")
    ));
    rdr.deserialize()
        .map(|result| result.map(|(a, b, c): (f64, f64, f64)| Measurement(a, b, c)))
        .try_collect()
}

fn new_input(path: PathBuf) -> Result<Vec<Measurement>, csv::Error> {
    if io::stdin().is_terminal() {
        deserialise(path)
    } else {
        let mut rdr = csv::Reader::from_reader(io::stdin());
        let input: Result<Vec<Measurement>, csv::Error> = rdr
            .deserialize()
            .map(|result| result.map(|(a, b, c): (f64, f64, f64)| Measurement(a, b, c)))
            .try_collect();
        input
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_relative_eq;
    use nalgebra as na;
    use na::Point2;
    #[test]
    fn translation() {
        let plane = Plane::from_xy(&[
            Point2::new(0.0, 0.0),
            Point2::new(0.0, 10.0),
            Point2::new(10.0, 0.0),
        ]);
        let plane1 = plane.new_reading(&Measurement(1.0, 1.0, 1.0));
        let plane2 = plane.new_reading(&Measurement(1.3, 1.3, 1.3));
        let isometry = plane1.isometry_from(&plane2).to_matrix();
        let mut translation = na::Matrix4::identity();
        translation.m34 = 0.3;

        assert_relative_eq!(isometry, translation);
    }
    #[test]
    fn reads_calibration() {
        // let mut rdr = csv::Reader::from_reader(rdr)
        let calibration =
            deserialise("calibration.csv".into()).expect("Unable to start calibration");
        let test = [
            Measurement(1.0, 1.0, 1.0),
            Measurement(1.05, 1.0, 1.0),
            Measurement(1.03, 1.0, 1.0),
        ];
        assert_eq!(calibration[0], test[0])
    }

    #[test]
    fn input() {
        let base_plane = Plane::from_xy(&[
            Point2::new(0.0, 0.0),
            Point2::new(0.0, 10.0),
            Point2::new(10.0, 0.0),
        ]);
        let input = [
            Measurement(1.0, 1.0, 1.0),
            Measurement(1.2, 1.0, 1.0),
            Measurement(1.3, 1.3, 1.3),
        ];
        let plane = base_plane.new_reading(&Measurement(1.0, 1.0, 1.0));
        let plane1 = plane.new_reading(&input[0]); // Nothing moved
        let translation = plane.new_reading(&input[2]);
        // let rotation = plane.new_reading(&input[1]);

        let isometry0 = plane.isometry_from(&plane1);
        assert_relative_eq!(isometry0, na::Isometry3::translation(0.0, 0.0, 0.0));
        let isometry1 = plane.isometry_from(&translation);
        assert_relative_eq!(isometry1, na::Isometry3::translation(0.0, 0.0, 0.3));
    }
}
