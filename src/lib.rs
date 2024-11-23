use std::io::prelude::*;
use std::str::FromStr;
use plotters::prelude::*;
use plotters::backend::BitMapBackend;
use plotters::chart::ChartBuilder;
use plotters::element::PathElement;
use plotters::prelude::{IntoFont, LineSeries, BLACK, RED, WHITE};

pub fn parse_arg<T: FromStr>(arg: &str, description: &str) -> T {
    arg.parse::<T>().unwrap_or_else(|_| {
        eprintln!("Invalid {}: {}", description, arg);
        std::process::exit(1);
    })
}

pub fn read_param<T: FromStr>(prompt: &str) -> T {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<T>() {
            Ok(value) => return value,
            Err(_) => {
                print!("Invalid input. Please try again: ");
                std::io::stdout().flush().unwrap();
            }
        }
    }
}

fn plotters_test() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plotters-doc-data/0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            &RED,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
