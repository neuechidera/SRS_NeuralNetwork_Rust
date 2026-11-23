use std::fs::File;
use std::io::{self, BufRead, BufReader};
use rand::{thread_rng, seq::SliceRandom};


// Reads the data sets into vectors

pub fn load_csv(filename: &str) -> (Vec<Vec<f64>>, Vec<f64>){
    let reader = BufReader::new(File::open(filename).unwrap());
    let mut inputs: Vec<Vec<f64>> = Vec::new();
    let targets: Vec<f64> = Vec::new();

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

// Rust doesn't let you make global constant variables so they have to be wrapped into a function or a struct
pub struct SplitData {
    inputs: Vec<Vec<f64>>,
    targets: Vec<f64>,
    train_inputs: Vec<Vec<f64>>,
    test_inputs: Vec<Vec<f64>>,
    train_targets: Vec<f64>,
    test_targets: Vec<f64>,
}

impl SplitData {
    pub fn split_data(&mut self, train_ratio: f64) { // Rust doesn't allow for default values
        // Clearing is similar to C++
        self.train_inputs.clear();
        self.train_targets.clear();
        self.test_inputs.clear();
        self.test_targets.clear();

        //makes indices contain a vector of randomly shuffled numbers
        let mut indices: Vec<usize> = (0..self.inputs.len()).collect();
        let mut rd = rand::thread_rng();
        indices.shuffle(&mut rd);

        // size() is the same as len() in Rust
        let train_size: usize = (train_ratio * self.inputs.len() as f64) as usize;

        //for loop adds the columns from the file randomly into the test and training sets
        for i in 0..self.inputs.len() {
            let shuffle_index = indices[i];

            if i < train_size {
                self.train_inputs.push(self.inputs[shuffle_index].clone());
                self.train_targets.push(self.targets[shuffle_index]);
            }else{
                self.test_inputs.push(self.inputs[shuffle_index].clone());
                self.test_targets.push(self.targets[shuffle_index]);
            }
        }
    }
}
