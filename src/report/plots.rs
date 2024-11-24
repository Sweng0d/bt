use plotters::prelude::*;
//use crate::data_loader::MarketData;

pub fn plot_portfolio_value(portfolio_values: &[f64], file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(file_path, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_axis: Vec<usize> = (0..portfolio_values.len()).collect();

    let mut chart = ChartBuilder::on(&root)
        .caption("Evolução do Valor da Carteira", ("sans-serif", 50).into_font())
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(70)
        .build_cartesian_2d(0..portfolio_values.len(), *portfolio_values.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()..*portfolio_values.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap())?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        x_axis.iter().zip(portfolio_values.iter()).map(|(x, y)| (*x, *y)),
        &BLUE,
    ))?
    .label("Valor da Carteira")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart.configure_series_labels().background_style(&WHITE.mix(0.8)).draw()?;

    Ok(())
}
