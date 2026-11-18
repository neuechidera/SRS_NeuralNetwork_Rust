use std::fs::File;
use std::io::{self, BufRead, BufReader};


// Reads the data sets into vectors

pub fn load_csv(filename: &str) -> (Vec<Vec<f64>>, Vec<f64>){
    let line: String;
    let reader = BufReader::new(File::open(filename).unwrap());
    let mut inputs: Vec<Vec<f64>> = Vec::new();
    let mut targets: Vec<f64> = Vec::new();

    for line in reader.lines().skip(1) {
        let line = line?;
        let row = line.split(",").filter_map(|val| val.trim().parse::<f64>().ok()).collect();

        inputs.push(row);
    }

    (inputs, targets)
}
