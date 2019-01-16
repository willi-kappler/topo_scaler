use std::fs::File;
use std::io::{BufReader, BufWriter, BufRead, Write};

use crate::error::ProcessError;

pub fn process_config(options: super::CLOptions) -> Result<(), ProcessError> {
    let file = File::open(options.input_file)?;
    let input_file = BufReader::new(file);

    let cell_size: usize = (1.0 / options.scale_factor).round() as usize;

    let output_cols: usize = options.num_of_cols / cell_size;
    let output_rows: usize = options.num_of_rows / cell_size;

    let mut output_topography: Vec<f64> = Vec::with_capacity((output_cols * output_rows) as usize);

    let mut input_x: usize = 0;
    let mut input_y: usize = 0;

    let mut cell_x: usize = 0;
    let mut cell_y: usize = 0;

    let mut output_x: usize = 0;
    let mut output_y: usize = 0;

    for line in input_file.lines() {
        let value: f64 = line?.parse()?;

        output_topography[(output_y * output_cols) + output_x] += value;

        cell_x += 1;
        if cell_x >= cell_size {
            cell_x = 0;
            output_x += 1;
        }

        input_x += 1;
        if input_x >= options.num_of_cols {
            output_x = 0;

            cell_x = 0;
            cell_y += 1;

            if cell_y >= cell_size {
                cell_y = 0;
                output_y += 1;
            }

            input_x = 0;
            input_y += 1;
            if input_y >= options.num_of_rows {
                break;
            }
        }
    }

    let scaled_topography =  output_topography.iter().map(|value| value / ((cell_size * cell_size) as f64));

    let file = File::open(options.output_file)?;
    let mut output_file = BufWriter::new(file);

    for value in scaled_topography {
        write!(output_file, "{}\n", value)?;
    }

    Ok(())
}
