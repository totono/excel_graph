mod data;

use plotly::layout::{
    Axis, Layout, Margin,
};

use plotly::{Plot, Scatter,ImageFormat};

pub fn line_and_scatter_plot() {
    let records = data::get_data().ok().unwrap();

    let date: Vec<String> = records.iter().map(| x| x.time.clone()).collect();
    let y: Vec<f32> = records.iter().map(|y| 100.0 - y.percent).collect(); 

    let trace1 = Scatter::new(date, y)
        .name("空き時間");

    let layout = Layout::new()
    .margin(Margin::new().top(20).bottom(130).left(50).right(50))
    .y_axis(Axis::new().range(vec![0.0,100.0]).n_ticks(20))
    .x_axis(Axis::new().n_ticks(60).tick_angle(270.0));

    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(trace1);
    plot.save("test",ImageFormat::PNG, 1240,500,1.0);
    //plot.show();
}

fn main() -> std::io::Result<()> {
    line_and_scatter_plot();
    Ok(())
}
