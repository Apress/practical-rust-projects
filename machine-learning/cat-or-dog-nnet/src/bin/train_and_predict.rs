extern crate rusty_machine;
extern crate rand;

use std::error::Error;
use structopt::StructOpt;
use serde::Deserialize;
use csv;

use rusty_machine::learning::nnet::{NeuralNet, BCECriterion};
use rusty_machine::learning::optim::grad_desc::StochasticGD;

use rusty_machine::linalg::Matrix;
use rusty_machine::learning::SupModel;
use rusty_machine::data::transforms::{Transformer, Standardizer};

#[derive(StructOpt)]
struct Options {
    #[structopt(short = "r", long = "train", parse(from_os_str))]
    /// Training data CSV file
    training_data_csv: std::path::PathBuf,

    #[structopt(short = "t", long = "test", parse(from_os_str))]
    /// Testing data CSV file
    testing_data_csv: std::path::PathBuf,
}

#[derive(Debug, Deserialize)]
struct SampleRow {
    height: f64,
    length: f64,
    category_id: usize,
}

fn read_data_from_csv(file_path: std::path::PathBuf)
    -> Result<(Matrix<f64>, Matrix<f64>), Box<dyn Error>> {
    let mut input_data = vec![];
    let mut label_data = vec![];
    let mut sample_count = 0;
    let mut reader = csv::Reader::from_path(file_path)?;
    for raw_row in reader.deserialize() {
        let row: SampleRow = raw_row?;
        input_data.push(row.height);
        input_data.push(row.length);
        label_data.push(row.category_id as f64);
        sample_count += 1
    }

    let inputs = Matrix::new(sample_count, 2, input_data);
    let targets = Matrix::new(sample_count, 1, label_data);
    return Ok((inputs, targets))
}


fn main() -> Result<(), Box<dyn Error>>{

    let options = Options::from_args();

    // load the data from CSV
    let (training_inputs, targets) = read_data_from_csv(options.training_data_csv).unwrap();

    // Training ====================
    // normalization
    let mut standardizer = Standardizer::new(0.0, 1.0);

    standardizer.fit(&training_inputs).unwrap();
    let normalized_training_inputs = standardizer.transform(training_inputs).unwrap();
    // println!("{:?}", normalized_training_inputs);

    let layers = &[2, 2, 1];
    // http://www.deepideas.net/deep-learning-from-scratch-iii-training-criterion/
    //let criterion = BCECriterion::new(Regularization::L2(0.));
    //let criterion = BCECriterion::default();
    // Create a multilayer perceptron with an input layer of size 2 and output layer of size 1
    // Uses a Sigmoid activation function and uses Stochastic gradient descent for training
    //let gradient_descent = StochasticGD::new(0.1, 0.1, 20);
    let mut model = NeuralNet::default(layers);

    model.train(&normalized_training_inputs, &targets).unwrap();

    // Testing ====================
    let (testing_inputs, expected) = read_data_from_csv(options.testing_data_csv).unwrap();

    // Normalize the testing data using the mean and variance of the training data
    let normalized_test_cases = standardizer.transform(testing_inputs).unwrap();
    // println!("{:?}", normalized_test_cases);

    let res = model.predict(&normalized_test_cases).unwrap();

    // Calculating accuracy =================
    println!("{:?}", res);

    println!("Evaluation...");
    let mut hits = 0;
    let mut misses = 0;
    // Evaluation
    println!("Got\tExpected");
    for (idx, prediction) in res.into_vec().iter().enumerate() {
        println!("{:.2}\t{}", prediction, expected[[idx, 0]]);
        if (prediction - 0.5) * (expected[[idx, 0]] - 0.5) > 0. {
            hits += 1;
        } else {
            misses += 1;
        }
    }

    println!("Hits: {}, Misses: {}", hits, misses);
    let hits_f = hits as f64;
    let total = (hits + misses) as f64;
    println!("Accuracy: {}%", (hits_f / total) * 100.);
    Ok(())
}

