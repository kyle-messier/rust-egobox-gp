use clap::Parser;
use std::path::PathBuf;

mod gp_model;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Fit model and predict
    let predictions = gp_model::fit_and_predict(&args.input_csv, &args.predict_csv)?;
    
    // Save predictions
    gp_model::save_predictions(&args.output_csv, &predictions)?;

    Ok(())
}
