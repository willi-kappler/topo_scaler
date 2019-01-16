use structopt::StructOpt;

mod process_topography;
mod error;

use crate::process_topography::process_config;
use crate::error::ProcessError;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct CLOptions {
    /// Number of rows in the input file
    #[structopt(long = "rows")]
    num_of_rows: usize,

    /// Number of columns in the input file
    #[structopt(long = "cols")]
    num_of_cols: usize,

    /// Scale factor
    #[structopt(long = "scale")]
    scale_factor: f64,

    /// Input topography file
    #[structopt(long = "input")]
    input_file: String,

    /// Output topography file
    #[structopt(long = "output")]
    output_file: String,
}

fn main() {
    let options = CLOptions::from_args();

    match process_config(options) {
        Ok(()) => {
            println!("Processing finished without errors");
        }
        Err(e) => {
            println!("The following error occurred: {}", e);
        }
    }
}
