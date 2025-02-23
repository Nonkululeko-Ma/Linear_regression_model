fn train_model(model: &mut LinearRegression, data: &[(f32, f32)], epochs: usize, lr: f32) {
    for _ in 0..epochs {
        let mut loss = 0.0;

        for &(x, y) in data {
            let x_tensor = Tensor::from_data(Data::from([x]));
            let y_true = Tensor::from_data(Data::from([y]));
            let y_pred = model.forward(&x_tensor);

            let error = &y_pred - &y_true;
            let grad = error.clone() * 2.0; // Gradient descent step

            model.weight = &model.weight - &grad * lr;
            model.bias = &model.bias - &grad * lr;
            loss += (error.clone() * error.clone()).to_data().value[0];
        }

        println!("Loss: {}", loss);
    }
}
