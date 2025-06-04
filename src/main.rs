use clap::Parser;
use std::path::{Path, PathBuf};
use friedrich::gaussian_process::GaussianProcessBuilder;
use friedrich::kernel::Matern2;
use friedrich::prior::Prior;
use friedrich::prior::LinearPrior;
use csv::{WriterBuilder};
use serde::Serialize;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(name = "rustgp")]
struct Args {
    #[arg(long)]
    input_csv: PathBuf,

    #[arg(long)]
    predict_csv: PathBuf,

    #[arg(long)]
    output_csv: PathBuf,
}

#[derive(Debug, Serialize)]
pub struct Prediction {
    pub x: f64,
    pub y: f64,
    pub predicted_mean: f64,
    pub predicted_variance: f64,
}

pub fn save_predictions(output_csv: &Path, predictions: &[Prediction]) -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new().from_path(output_csv)?;
    for pred in predictions {
        wtr.serialize(pred)?;
    }
    wtr.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    
    // Read training data using CSV and perform Gaussian Process regression
    let mut rdr = csv::ReaderBuilder::new().from_path(&args.input_csv)?;
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let x: f64 = record[0].parse()?;
        let y: f64 = record[1].parse()?;
        let val: f64 = record[2].parse()?;
        inputs.push(vec![x, y]);
        outputs.push(val);
    }

 
    // Set up the Gaussian Process model using the GaussianProcessBuilder struct from `friedrich`
    let input_dimension = 2; // Assuming 2D inputs
    let output_noise = 0.1; // Example noise level
    let matern_kernel = Matern2::default();
    let linear_prior = LinearPrior::default(input_dimension);
    let mut gp = GaussianProcessBuilder::<Matern2, LinearPrior>::new(inputs, outputs)
        .set_noise(output_noise)
        .set_kernel(matern_kernel)
        .fit_kernel()
        .set_prior(linear_prior)
        .fit_prior()
        .train();


        let fit_prior = true;
        let fit_kernel = true;
        let max_iter = 1000;
        let convergence_fraction = 1e-6;
        let max_time = std::time::Duration::from_secs(3600);
        gp.fit_parameters(fit_prior, fit_kernel, max_iter, convergence_fraction, max_time);   

    // Read prediction locations
    let mut pred_rdr = csv::ReaderBuilder::new().from_path(&args.predict_csv)?;
    let mut prediction_inputs = Vec::new();
    for result in pred_rdr.records() {
        let record = result?;
        let x: f64 = record[0].parse()?;
        let y: f64 = record[1].parse()?;
        prediction_inputs.push(vec![x, y]);
    }


    // Perform predictions from 
    // the Gaussian Process model
    let mut predictions = Vec::new();
    for input in prediction_inputs {
        let (mean, var) = gp.predict_mean_variance(&input);
        predictions.push(Prediction {
            x: input[0],
            y: input[1],
            predicted_mean: mean,
            predicted_variance: var,
        });
    }

        // Borrow the kernel from the GP model
        let kernel = &gp.kernel; 

        // Print kernel details
        println!("Fitted kernel: {:?}", kernel);

        // Borrow the noise parameter
        let noise = &gp.noise;
        // Print noise details
        println!("Fitted noise: {}", noise);

    // Save predictions
    save_predictions(&args.output_csv, &predictions)?;

    Ok(())
}

// fn main() {
//         let training_inputs = vec![
//             vec![10.0, 0.0],
//             vec![15.0, 0.0],
//             vec![20.0, 1.0],
//             vec![10.0, 1.0],
//         ];
//         let training_outputs = vec![3.0, 1.0, 1.0, 1.0];


//                 // Model parameters.
//         let input_dimension = 2;
//         let output_noise = 0.001;
//         let exponential_kernel = Matern2::default();
//         let linear_prior = LinearPrior::default(input_dimension);

//         // Defining and training a model.
//         let mut gp = GaussianProcess::builder(training_inputs, training_outputs)
//             .set_noise(output_noise)
//             .set_kernel(exponential_kernel)
//             .fit_kernel()
//             .set_prior(linear_prior)
//             .fit_prior()
//             .train();
//         // Optional: fit hyperparameters.
//         let fit_prior = true;
//         let fit_kernel = true;
//         let max_iter = 100;
//         let convergence_fraction = 1e-5;
//         let max_time = std::time::Duration::from_secs(3600);
//         gp.fit_parameters(fit_prior, fit_kernel, max_iter, convergence_fraction, max_time);
//         // Read prediction locations.
//         let prediction_inputs = vec![
//             vec![10.5, 0.5],
//             vec![10.0, 0.0],
//         ];
//         let mut predictions = Vec::new();
//         for input in prediction_inputs {
//             let mean = gp.predict(&input);
//             let var = gp.predict_variance(&input);
//             predictions.push((input, mean, var));
//         }
//         // Print predictions.
//         for (input, mean, var) in predictions {
//             println!("Input: {:?}, Predicted Mean: {}, Predicted Variance: {}", input, mean, var);
//         }

//         // Borrow the kernel from the GP model
//         let kernel = &gp.kernel; 

//         // Print kernel details
//         println!("Fitted kernel: {:?}", kernel);

//         // Borrow the noise parameter
//         let noise = &gp.noise;
//         // Print noise details
//         println!("Fitted noise: {}", noise);

// }