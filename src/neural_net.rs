use rand::Rng;

pub fn sigmoid(x: f64) -> f64 {
    return 1.0 / (1.0 + (-x).exp());
}

pub fn sigmoid_derivative(x: f64) -> f64 {
    return 1.0 / (1.0 + (-x).exp());
}


struct NeuralNet{
    input_size: usize,
    hidden1_size: usize,
    hidden2_size: usize,
    weights_input_hidden1: Vec<f64>,
    weights_hidden1_hidden2: Vec<f64>,
    weights_hidden2_output: Vec<f64>,
}

impl NeuralNet {
    fn rand_weight() -> f64{
        rand::thread_rng().gen_range(-0.5..0.5)
    }

    pub fn new(input_size: usize, hidden1_size: usize, hidden2_size: usize) -> Self {
        let mut weights_input_hidden1 = Vec::new();
        let mut weights_hidden1_hidden2 = Vec::new();
        let mut weights_hidden2_output = Vec::new();
        
        weights_input_hidden1.resize_with(input_size * hidden1_size, Self::rand_weight);

        if hidden2_size > 0 {
            // Weights are initialized to connect hidden layer 1 to hidden layer 2
            weights_hidden1_hidden2.resize_with(hidden2_size * hidden1_size, Self::rand_weight);
            weights_hidden2_output.resize_with(hidden2_size, Self::rand_weight); // Connect Hidden Layer 2 to Output
        } else {
            weights_hidden2_output.resize_with(hidden1_size, Self::rand_weight);
        }
        
        NeuralNet {
            input_size,
            hidden1_size,
            hidden2_size,
            weights_hidden1_hidden2,
            weights_input_hidden1,
            weights_hidden2_output,
        }
    }

    pub fn train(&mut self, inputs: &Vec<Vec<f64>>, targets: &Vec<f64>, epochs: i32, lr: f64) {
        let error: f64;
        let mut output: f64;
        let d_output: f64;

        for epoch in 0..epochs {
            for i in 0..inputs.len(){
                let mut hidden1: Vec<f64> = Vec::with_capacity(self.hidden1_size);
                for j in 0..self.hidden1_size {
                    hidden1[j] = 0.0;

                    for k in 0..self.input_size {
                        hidden1[j] += inputs[i][k] * self.weights_input_hidden1[j * self.input_size + k]
                    }
                    hidden1[j] = sigmoid(hidden1[j]);
                }

                let mut hidden2: Vec<f64>

                if (self.hidden2_size > 0) {
                    hidden2 = Vec::with_capacity(self.hidden2_size);

                    for j in 0..self.hidden2_size {
                        hidden2[j] = 0.0;

                        for k in 0..self.hidden1_size {
                            hidden2[j] += hidden1[k] * self.weights_hidden1_hidden2[j * self.hidden1_size + k];
                        }

                        hidden2[j] = sigmoid(hidden2[j]);
                    }
                }
            }

            output = 0.0;

            let final_hidden: &Vec<f64> = if self.hidden2_size > { &hidden2 } else{ &hidden1 };
            
        }
    }
}
