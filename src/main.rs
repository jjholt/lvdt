extern crate approx;
extern crate nalgebra as na;

use clap::Parser;
use csv::Reader;
use std::{
    fs,
    io::{self, IsTerminal},
    path::Path,
};
mod plot;

mod models;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Parser)]
struct Args {
    /// Calibration data
    #[arg(short,long, default_value_t = String::from("calibration.csv"))]
    calibration: String,
    /// Data
    #[arg(short,long, default_value_t = String::from("data.csv"))]
    data: String,
}

fn main() {
    let args = Args::parse();

    let calibration = deserialise(Path::new(&args.calibration))
        .unwrap_or_else(|e| panic!("Missing calibration file: {e:?}"));
    let input = extract_input(Path::new(&args.data));

    let config =
        get_config("config.yaml").unwrap_or_else(|e| panic!("Unable to load configuration: {e:?}"));
    let base_plane = config.screws;
    let implant = config.implant;

    let screw = base_plane.new_reading(&average(&calibration));

    let mut wtr = csv::Writer::from_writer(io::stdout());

    match input {
        Ok(measurement) => measurement
            .iter()
            .map(|m| screw.new_reading(m))
            .map(|p| screw.isometry_from(&p))
            .map(|i| implant.apply_isometry(&i))
            .map(|p| implant.isometry_from(&p))
            .map(|f| models::Output::new(&f))
            .for_each(|f| wtr.serialize(f).unwrap()),
        Err(err) => {
            eprintln!("Unable to open input -- {}: {}", &args.data, err);
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

fn get_config(path: &str) -> Result<models::Config> {
    let yaml = fs::read_to_string(path)?;
    Ok(serde_yaml::from_str(&yaml)?)
}

fn average(measurements: &[models::Measurement]) -> models::Measurement {
    let len = measurements.len() as f64;
    let v = measurements.iter().fold((0.0, 0.0, 0.0), |acc, m| {
        (acc.0 + m.0 / len, acc.1 + m.1 / len, acc.2 + m.2 / len)
    });
    models::Measurement(v.0, v.1, v.2)
}

fn deserialise(path: &Path) -> Result<Vec<models::Measurement>> {
    let rdr = csv::Reader::from_path(&path)?;
    let measurements: Vec<models::Measurement> = deserialise_reader(rdr);
    if measurements.is_empty() {
        Err("No valid measurements found.".into())
    } else {
        Ok(measurements)
    }
}
fn deserialise_reader<T: io::Read>(mut rdr: Reader<T>) -> Vec<models::Measurement> {
    rdr.deserialize()
        .filter_map(|f| f.ok()) //this is shit
        .map(models::Measurement::new)
        .collect()
}

fn extract_input(path: &Path) -> Result<Vec<models::Measurement>> {
    if io::stdin().is_terminal() {
        deserialise(path)
    } else {
        let rdr = csv::Reader::from_reader(io::stdin());
        Ok(deserialise_reader(rdr))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_relative_eq;
    use models::Measurement;
    use models::Plane;
    use na::Point2;
    use nalgebra as na;
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
            deserialise(Path::new("calibration.csv")).expect("Unable to start calibration");
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
