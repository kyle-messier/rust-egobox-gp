use csv::{ReaderBuilder, WriterBuilder};
use serde::{Serialize};
use std::error::Error;
use std::path::Path;
use friedrich::gaussian_process::GaussianProcess;

#[derive(Debug, Serialize)]
pub struct Prediction {
    pub x: f64,
    pub y: f64,
    pub predicted_mean: f64,
    pub predicted_variance: f64,
}


pub fn fit_and_predict(
    input_csv: &Path,
    predict_csv: &Path,
) -> Result<Vec<Prediction>, Box<dyn Error>> {
    // Read the training data
    let mut rdr = ReaderBuilder::new().from_path(input_csv)?;
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

    let mut gp: GaussianProcess<friedrich::kernel::SquaredExp, friedrich::prior::ConstantPrior> =
        GaussianProcess::default(inputs, outputs);

    let fit_prior = true;
    let fit_kernel = true;
    let max_iter = 100;
    let convergence_fraction = 0.00001;
    let max_time = std::time::Duration::from_secs(3600);

    // Fit model parameters
    gp.fit_parameters(fit_prior, fit_kernel, max_iter, convergence_fraction, max_time);
    println!("lengthscale: {:?}", gp.kernel.ls);
    println!("sill: {:?}", gp.kernel.ampl);


    // Read prediction locations
    let mut pred_rdr = ReaderBuilder::new().from_path(predict_csv)?;
    let mut predictions = Vec::new();

    for result in pred_rdr.records() {
        let record = result?;
        let x: f64 = record[0].parse()?;
        let y: f64 = record[1].parse()?;
        let input = vec![x, y];
        let mean = gp.predict(&input);
        let var = gp.predict_variance(&input);


        predictions.push(Prediction {
            x,
            y,
            predicted_mean: mean,
            predicted_variance: var,
        });
    }

    Ok(predictions)
}

pub fn save_predictions(output_csv: &Path, predictions: &[Prediction]) -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new().from_path(output_csv)?;
    for pred in predictions {
        wtr.serialize(pred)?;
    }
    wtr.flush()?;
    Ok(())
}
