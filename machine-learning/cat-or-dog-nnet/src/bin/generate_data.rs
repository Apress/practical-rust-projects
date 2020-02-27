extern crate rusty_machine;
extern crate rand;

use rusty_machine::linalg::{Matrix, BaseMatrix};

use rand::thread_rng;
use rand::distributions::Distribution; // for using .sample()
use rand_distr::Normal; // splitted from rand since 0.7
use std::io;
use serde::Serialize;

// settings
const CENTROIDS:[f64;4] =  [ // Height, length
    61.0, 99.5, // German Shepherd dog
    22.5, 40.5, // Persian cat
];

const NOISE:f64 = 1.8;
const SAMPLES_PER_CENTROID: usize = 2000;

#[derive(Debug, Serialize)]
struct Sample {
    height: f64,
    length: f64,
    category_id: usize
}

fn generate_data(centroids: &Matrix<f64>,
                 points_per_centroid: usize,
                 noise: f64)
                 -> Vec<Sample> {
    assert!(centroids.cols() > 0, "Centroids cannot be empty.");
    assert!(centroids.rows() > 0, "Centroids cannot be empty.");
    assert!(noise >= 0f64, "Noise must be non-negative.");
    let mut samples = Vec::with_capacity(points_per_centroid);

    let mut rng = thread_rng();
    let normal_rv = Normal::new(0f64, noise).unwrap();

    for _ in 0..points_per_centroid {
        // Generate points from each centroid
        for (centroid_id, centroid) in centroids.iter_rows().enumerate() {
            let mut point = Vec::with_capacity(centroids.cols());
            for feature in centroid.iter() {
                point.push(feature + normal_rv.sample(&mut rng));
            }

            samples.push(Sample {
                height: point[0],
                length: point[1],
                category_id: centroid_id,
            });
        }
    }

    samples
}

fn main() -> Result<(), std::io::Error> {
    let centroids = Matrix::new(2, 2, CENTROIDS.to_vec());

    let samples = generate_data(&centroids, SAMPLES_PER_CENTROID, NOISE);

    let mut writer = csv::Writer::from_writer(io::stdout());
    // serialize will generate the column header automatically
    for sample in samples.iter() {
        writer.serialize(sample)?;
    }
    Ok(())
}
