use ahrs::{Ahrs, Madgwick};
use nalgebra::Vector3;
use std::f64;
use std::io::{self};

fn main() {
    // Initialize filter with default values
    let mut ahrs = Madgwick::default();

    let stdin = io::stdin();
    for line in stdin.lines() {
        let values = line
            .unwrap()
            .split(',')
            .map(|x| x.trim().parse::<f64>())
            .collect::<Result<Vec<f64>, _>>()
            .unwrap();
        if values.len() != 9 {
            println!("Got strange line {:?}", values);
            continue;
        }

        let accelerometer = Vector3::new(values[0], values[1], values[2]);
        let gyroscope = Vector3::new(values[3], values[4], values[5]);
        let magnetometer = Vector3::new(values[6], values[7], values[8]);

        let quat = ahrs
            .update(
                &(gyroscope * (f64::consts::PI / 180.0)),
                &accelerometer,
                &magnetometer,
            )
            .unwrap();
        let (roll, pitch, yaw) = quat.euler_angles();

        println!("pitch={}, roll={}, yaw={}", pitch, roll, yaw);
    }
}
