use burn::tensor::backend::Backend;
use burn::tensor::{Tensor, Data};
use burn::module::Module;

#[derive(Module, Debug)]
struct LinearRegression {
    weight: Tensor<f32>,
    bias: Tensor<f32>,
}

impl LinearRegression {
    fn new() -> Self {
        Self {
            weight: Tensor::from_data(Data::from([0.0])),
            bias: Tensor::from_data(Data::from([0.0])),
        }
    }

    fn forward(&self, x: &Tensor<f32>) -> Tensor<f32> {
        &self.weight * x + &self.bias
    }
}
