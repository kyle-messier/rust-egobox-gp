use clap::Parser;
use std::path::{Path, PathBuf};
use friedrich::gaussian_process::GaussianProcessBuilder;
use friedrich::kernel::Exponential;
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
    let output_noise = 0.0001; 
    // Create the Gaussian Process model with the specified kernel and prior
    let exp_kernel = Exponential::default();
    let linear_prior = LinearPrior::default(input_dimension);
    let gp = GaussianProcessBuilder::<Exponential, LinearPrior>::new(inputs, outputs)
        .set_noise(output_noise)
        .set_kernel(exp_kernel)
        .fit_kernel()
        .set_prior(linear_prior)
        .fit_prior()
        .train();



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
