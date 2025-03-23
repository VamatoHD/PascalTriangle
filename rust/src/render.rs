use plotters::prelude::*;

pub fn render(data: &Vec<Vec<(f32, f32)>>, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(output, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let width = data
        .iter()
        .flat_map(|v| v.iter().map(|(x, _)| *x))
        .fold(f32::MIN, f32::max);
    let height = data
        .iter()
        .flat_map(|v| v.iter().map(|(_, y)| *y))
        .fold(f32::MIN, f32::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("Pascal Triangle Performance", ("Arial", 30).into_font())
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f32..(width + 1.0), 0f32..(height + 1.0))?;

    chart
        .configure_mesh()
        .x_desc("Linha")
        .y_desc("Tempo (ms)")
        .axis_desc_style(("sans-serif", 20).into_font())
        .draw()?;

    let colors = vec![RED, BLACK];

    for i in 0..data.len() {
        let c = colors[i].stroke_width(3);
        chart
            .draw_series(LineSeries::new(data[i].clone(), c.clone()))?
            .label(i.to_string())
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], c.clone()));
    }

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .draw()?;

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
