use textplots::{Chart, Plot, Shape};

fn plot_results(data: &[(f32, f32)], model: &LinearRegression) {
    let plot_data: Vec<(f32, f32)> = data.iter()
        .map(|&(x, _)| (x, model.forward(&Tensor::from_data(Data::from([x]))).to_data().value[0]))
        .collect();

    Chart::new(100, 30, 0.0, 10.0)
        .lineplot(&Shape::Points(&plot_data))
        .display();
}
