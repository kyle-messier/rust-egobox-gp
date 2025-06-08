use clap::Parser;
use std::path::{Path, PathBuf};
use std::error::Error;

use egobox_gp::{
    correlation_models::*,
    mean_models::*,
    GaussianProcess,
};
use linfa::prelude::*;
use csv::WriterBuilder;
use serde::Serialize;
use ndarray::{Array1, Array2};
use ndarray::array;
#[derive(Parser, Debug)]
#[command(name = "rust-egobox-gp")]
struct Args {
    #[arg(long)] input_csv:   PathBuf,
    #[arg(long)] predict_csv: PathBuf,
    #[arg(long)] output_pred_csv: PathBuf,
    #[arg(long)] output_kernel_csv: Option<PathBuf>,
    #[arg(long)] output_noise_csv:  Option<PathBuf>,
}

#[derive(Debug, Serialize)]
pub struct Prediction {
    pub x: f64,
    pub y: f64,
    pub predicted_mean: f64,
    pub predicted_variance: f64,
}

fn save_predictions(path: &Path, preds: &[Prediction]) -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new().from_path(path)?;
    for p in preds {
        wtr.serialize(p)?;
    }
    wtr.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // ──────────── 1. TRAINING DATA ────────────
    let mut rdr = csv::ReaderBuilder::new().from_path(&args.input_csv)?;
    let mut xy_flat: Vec<f64> = Vec::new();   // (x,y) flattened
    let mut y_vec:   Vec<f64> = Vec::new();   // targets

    for rec in rdr.records() {
        let rec = rec?;
        xy_flat.extend_from_slice(&[rec[0].parse()?, rec[1].parse()?]);
        y_vec.push(rec[2].parse()?);
    }

    let n_samples = y_vec.len();
    let inputs  = Array2::from_shape_vec((n_samples, 2), xy_flat)?; // (N,2)
    let targets = Array1::from_vec(y_vec);                          // (N,)

    let dataset = Dataset::new(inputs, targets);

    // ──────────── 2. FIT GP ────────────
    let gp = GaussianProcess::<f64, LinearMean, AbsoluteExponentialCorr>::params(
                 LinearMean::default(),
                 AbsoluteExponentialCorr::default()
             )
            .theta_init(array![1.0])
            .nugget(0.1)
            .n_start(10)
             .fit(&dataset)
             .expect("GP fit");

    // ──────────── 3. PREDICTION LOCATIONS ────────────
    let mut pr_rdr = csv::ReaderBuilder::new().from_path(&args.predict_csv)?;
    let mut preds = Vec::new();

    for rec in pr_rdr.records() {
        let rec = rec?;
        let x = rec[0].parse::<f64>()?;
        let y = rec[1].parse::<f64>()?;

        // single row (1,2) array
        let x_arr = Array2::from_shape_vec((1, 2), vec![x, y])?;

        // predict returns length-1 Array1; var returns (1,1) Array2
        let m   = gp.predict(&x_arr)?[0];
        let var = gp.predict_var(&x_arr)?[[0, 0]];

        preds.push(Prediction { x, y, predicted_mean: m, predicted_variance: var });
    }

    // ──────────── 4. SAVE OUTPUTS ────────────
    save_predictions(&args.output_pred_csv, &preds)?;

    // ----- hyper-parameters -------------------------------------------------
    let theta = gp.theta().to_vec();
    let noise = gp.variance();

    // write kernel params (everything but the last slot)
    if let Some(k_path) = &args.output_kernel_csv {
        // drop the last element (=noise)
        std::fs::write(k_path, format!("{:?}", theta))?;
    }

    // write noise into its own file
    if let Some(n_path) = &args.output_noise_csv {
        std::fs::write(n_path, format!("{:.8}", noise))?;
    }


    Ok(())
}
