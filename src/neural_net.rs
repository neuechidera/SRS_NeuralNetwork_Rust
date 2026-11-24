use rand::Rng;

pub fn sigmoid(x: f64) -> f64 {
    return 1.0 / (1.0 + (-x).exp());
}

pub fn sigmoid_derivative(x: f64) -> f64 {
    return 1.0 / (1.0 + (-x).exp());
}

// Rust has no classes so we have to use a sturct to manage the types and data
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
        let mut error: f64;
        let mut output: f64;
        let mut d_output: f64;

        for epoch in 0..epochs {
            for i in 0..inputs.len(){
                let mut hidden1: Vec<f64> = vec![0.0; self.hidden1_size];
                for j in 0..self.hidden1_size {
                    for k in 0..self.input_size {
                        hidden1[j] += inputs[i][k] * self.weights_input_hidden1[j * self.input_size + k]
                    }
                    hidden1[j] = sigmoid(hidden1[j]);
                }

                let mut hidden2 = Vec::new();

                if self.hidden2_size > 0 {
                    hidden2.resize(self.hidden2_size, 0.0);

                    for j in 0..self.hidden2_size {
                        for k in 0..self.hidden1_size {
                            hidden2[j] += hidden1[k] * self.weights_hidden1_hidden2[j * self.hidden1_size + k];
                        }
                        hidden2[j] = sigmoid(hidden2[j]);
                    }
                }
                output = 0.0;
                let final_hidden: &Vec<f64> = if self.hidden2_size > 0 { &hidden2 } else { &hidden1 };
                for j in 0..final_hidden.len() {
                    output += final_hidden[j] * self.weights_hidden2_output[j];
                }
                output = sigmoid(output);

                // Back Propagation
                error = targets[i] - output; // Determine the error between prediction and actual

                d_output = error * sigmoid_derivative(output);

                for j in 0..final_hidden.len() {
                    self.weights_hidden2_output[j] += lr * d_output * final_hidden[j];
                }

                if self.hidden2_size > 0 {
                    let mut d_hidden2 = vec![0.0; self.hidden2_size];

                    for j in 0..self.hidden2_size {
                        d_hidden2[j] = d_output * self.weights_hidden2_output[j] * sigmoid_derivative(hidden2[j]);

                        for k in 0..self.hidden1_size {
                            self.weights_hidden1_hidden2[j * self.hidden1_size + k] += lr * d_hidden2[j] * hidden1[k];
                        }
                    }

                    for j in 0..self.hidden1_size {
                        let mut d_hidden1 = 0.0;

                        for k in 0..self.hidden2_size {
                            d_hidden1 += d_hidden2[k] * self.weights_hidden1_hidden2[k * self.hidden1_size + j];
                        }

                        d_hidden1 *= sigmoid_derivative(hidden1[j]);
                        for k in 0..self.input_size {
                            self.weights_input_hidden1[j * self.input_size + k] += lr * d_hidden1 * inputs[i][k];
                        }
                    }
                } else {
                    for j in 0..self.hidden1_size {
                        let d_hidden1 = d_output * self.weights_hidden2_output[j] * sigmoid_derivative(hidden1[j]);

                        for k in 0..self.input_size {
                            self.weights_input_hidden1[j * self.input_size + k] += lr * d_hidden1 * inputs[i][k];
                        }
                    }
                }
            }
        }
    }

    pub fn predict(&mut self, input: &Vec<f64>) -> f64 {
        let mut hidden1 = vec![0.0; self.hidden1_size];

        for j in 0..self.hidden1_size {
            for k in 0..self.input_size {
                hidden1[j] += input[k] * self.weights_input_hidden1[j * self.input_size + k];
            }
            hidden1[j] = sigmoid(hidden1[j]);
        }

        let mut hidden2: Vec<f64> = Vec::new();
        let final_hidden: &Vec<f64> = {
            if self.hidden2_size > 0 {
                hidden2.resize(self.hidden2_size, 0.0);
                for j in 0..self.hidden2_size {
                    for k in 0..self.hidden1_size {
                        hidden2[j] += hidden1[k] * self.weights_hidden1_hidden2[j * self.hidden1_size + k];
                    }
                    hidden2[j] = sigmoid(hidden2[j]);
                }
                &hidden2
            } else {
                &hidden1
            }
        }; // end of code block

        let mut output = 0.0;
        for j in 0..final_hidden.len() {
            output += final_hidden[j] * self.weights_hidden2_output[j];
        }

        sigmoid(output)
    }
}
