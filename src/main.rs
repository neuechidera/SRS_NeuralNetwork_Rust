mod functions;
use crate::functions::*;
mod neural_net;
use crate::neural_net::*;

use std::fs::File;

fn main() {
    let mut correct: i32;
    let mut true_positives: i32;
    let mut false_positives: i32;
    let mut false_negatives: i32;

    let mut prediction: f64;
    let mut accuracy: f64;
    let mut precision: f64;
    let (mut accuracy_total, mut precision_total): (f64, f64) = (0.0, 0.0);
    let mut acc_average: f64;
    let mut prec_average: f64;
    let (mut recall, mut f_one): (f64, f64) = (0.0, 0.0);
    let (mut recall_avg, mut recall_total, mut f_one_total, mut f_one_avg): (f64, f64, f64, f64) = (0.0, 0.0, 0.0, 0.0);

    let (mut inputs, targets) = load_csv("crossing_data.csv");
    normalize(&mut inputs);

    let mut splitter = SplitData::new(inputs, targets);
    
    for i in 0..20 {
        splitter.split_data(0.8);

        let mut nn = NeuralNet::new(7, 3, 0);
    }
}
