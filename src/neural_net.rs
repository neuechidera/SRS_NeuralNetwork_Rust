struct NeuralNet{
    input_size: usize,
    hidden1_size: usize,
    hidden2_size: usize,
    weights_input_hidden1: Vec<f64>,
    weights_hidden1_hidden2: Vec<f64>,
    weights_hidden2_output: Vec<f64>,
}

impl NeuralNet{
    // fn rand_weight() { // We have to import a file for rand
    //     rand
    }

    pub fn new(input_size: usize, hidden1_size: usize, hidden2_size: usize) -> Self {


        NeuralNet {
            input_size,
            hidden1_size,
            hidden2_size: 0,
            weights_hidden1_hidden2: Vec::new(),
            weights_input_hidden1: Vec::new(),
            weights_hidden2_output: Vec::new(),
        }

    }
}
