use std::fs::File;
use std::io::{self, BufRead, BufReader};


// Reads the data sets into vectors

pub fn load_csv(filename: &str) -> (Vec<Vec<f64>>, Vec<f64>){
    let line: String;
    let reader = BufReader::new(File::open(filename).unwrap());
    let mut inputs: Vec<Vec<f64>> = Vec::new();
    let mut targets: Vec<f64> = Vec::new();

    for line in reader.lines().skip(1) {
        let line = line.unwrap();
        let row = line.split(",").filter_map(|val| val.trim().parse::<f64>().ok()).collect();

        inputs.push(row);
    }

    (inputs, targets)
}

pub fn normalize_data(data: &mut Vec<Vec<f64>>) {
    let num_features: usize = data[0].len();
    let mut min_vals = vec![f64::MAX; num_features];
    let mut max_vals = vec![f64::MIN; num_features];

    for row in data.iter() {
        for i in 0..num_features {
            min_vals[i] = min_vals[i].min(row[i]);
            max_vals[i] = max_vals[i].max(row[i]);
        }
    }

}