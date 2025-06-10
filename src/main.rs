use clap::Parser;
use std::path::{Path, PathBuf};
use std::error::Error;
use csv::WriterBuilder;
use serde::Serialize;
use ndarray::{array, Array1, Array2};
use linfa::prelude::*;
use egobox_gp::{
    correlation_models::*,
    mean_models::*,
    GaussianProcess,
    ThetaTuning,
    ParamTuning,
};

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
    let mut xy_flat = Vec::<f64>::new();  // flattened (x,y)
    let mut y_vec   = Vec::<f64>::new();  // targets

    for rec in rdr.records() {
        let rec = rec?;
        xy_flat.push(rec[0].parse()?); // x
        xy_flat.push(rec[1].parse()?); // y
        y_vec.push(rec[2].parse()?);   // target
    }

    let n_samples = y_vec.len();
    let x: Array2<f64> = Array2::from_shape_vec((n_samples, 2), xy_flat)?;
    let y: Array1<f64> = Array1::from_vec(y_vec);

    let dataset = Dataset::new(x, y);



    // …

    let theta_tuning = ThetaTuning::Full {
        init: array![1.0],  // one shared θ
        bounds: array![(0.01, 10.0)], // bounds for the shared θ
    };

            

    // ----- GP build & fit --------------------------------------------
    let gp = GaussianProcess::<f64, LinearMean, AbsoluteExponentialCorr>::params(
                LinearMean::default(),
                AbsoluteExponentialCorr::default()
            )
            .kpls_dim(Some(1))
            .theta_tuning(theta_tuning)   
            .nugget(1.0)
            .n_start(100)
            .max_eval(10_000)
            .fit(&dataset)
            .expect("GP fit");


    // ──────────── 3. PREDICTION LOCATIONS ────────────
    let mut pr_rdr = csv::ReaderBuilder::new().from_path(&args.predict_csv)?;
    let mut preds = Vec::new();

    for rec in pr_rdr.records() {
        let rec = rec?;
        let x_pt = rec[0].parse::<f64>()?;
        let y_pt = rec[1].parse::<f64>()?;

        let x_arr = Array2::from_shape_vec((1, 2), vec![x_pt, y_pt])?;

        let mean = gp.predict(&x_arr)?[0];
        let var  = gp.predict_var(&x_arr)?[[0, 0]];

        preds.push(Prediction {
            x: x_pt,
            y: y_pt,
            predicted_mean: mean,
            predicted_variance: var,
        });
    }

    // ──────────── 4. SAVE OUTPUTS ────────────
    save_predictions(&args.output_pred_csv, &preds)?;

    // hyper-parameters
    let theta_vec = gp.theta();   // kernel params etc.
    let variance     = gp.variance();         // observation noise variance

    if let Some(k_path) = &args.output_kernel_csv {
        std::fs::write(k_path, format!("{:?}", theta_vec))?;
    }
    if let Some(n_path) = &args.output_noise_csv {
        std::fs::write(n_path, format!("{:.8}", variance))?;
    }

    Ok(())
}
