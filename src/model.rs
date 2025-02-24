use burn::prelude::*;
use burn::nn::{Linear};
use burn_ndarray::NdArray;



#[derive(Module, Debug)]
pub struct LinearRegression {
    layer: Linear<NdArray>,
}

impl LinearRegression {
    pub fn new() -> Self {
        Self {
            layer: Linear::new(1, 1),

        }
    }

    pub fn forward(&self, x: Tensor<NdArray, 1>) -> Tensor<NdArray, 1> {
        self.layer.forward(x)
    }
}

fn mean_squared_error(predictions: &Tensor<NdArray, 1>, targets: &Tensor<NdArray, 1>) -> Tensor<NdArray, 1> {
    let diff = predictions.sub(targets);
    let squared_diff = diff.mul(&diff);
    squared_diff.mean()
}

