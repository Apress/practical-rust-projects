use std::error::Error;
use std::io;
use gnuplot::{Figure, Caption, Graph};
use gnuplot::AxesCommon;

fn main() -> Result<(), Box<dyn Error>>{
    let mut x: Vec<f64> = Vec::new();
    let mut y: Vec<f64> = Vec::new();

    let mut reader = csv::Reader::from_reader(io::stdin());
    for result in reader.records() {
        let record = result?;
        x.push(record[0].parse().unwrap());
        y.push(record[1].parse().unwrap());
    }

    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Cat body measurements", &[])
        .set_legend(Graph(0.9), Graph(0.1), &[], &[])
        .set_x_label("height (cm)", &[])
        .set_y_label("length (cm)", &[])
        .points(x, y, &[Caption("Cat")]);
    fg.show();
    Ok(())
}
