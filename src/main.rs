mod functions;
use crate::functions::*;
mod neural_net;
use crate::neural_net::*;



fn main() {
    println!("Hello, world!");
    println!("Joshua Waters");
    let waters = sigmoid(5.83);
    println!("{:.2}", waters);
}
