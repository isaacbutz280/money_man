use eframe::egui::{plot, Response, Ui, Widget};

pub struct PieChart {
    wedges: Vec<plot::Polygon>,
}

impl Widget for PieChart {
    fn ui(self, ui: &mut Ui) -> Response {
        let plot = plot::Plot::new("items_demo")
            .legend(plot::Legend::default().position(plot::Corner::LeftTop))
            .allow_boxed_zoom(false)
            .allow_scroll(false)
            .allow_double_click_reset(false)
            .allow_zoom(false)
            .allow_drag(false)
            .show_background(false)
            .show_axes([false, false])
            .show_x(false)
            .show_y(false)
            .center_x_axis(true)
            .center_y_axis(true)
            .data_aspect(1.0)
            .auto_bounds_x()
            .auto_bounds_y();

        plot.show(ui, |plot_ui| {
            for pg in self.wedges {
                plot_ui.polygon(pg)
            }
        })
        .response
    }
}

impl PieChart {
    pub fn new(data: &[(&str, f64)]) -> Self {        
        let fidelity = 100;

        let sum = data.iter().fold(0.0, |acc, x| (acc + x.1));
        let mut start_angle;
        let mut end_angle = 0.0;

        let mut wedges = vec![];

        for d in data {
            start_angle = end_angle;
            end_angle = start_angle + (d.1 / sum) * 2.0 * std::f64::consts::PI;

            let l1 = plot::PlotPoints::from_parametric_callback(
                |t| (start_angle.cos() * t, start_angle.sin() * t),
                1.0..=0.0,
                fidelity,
            );

            let l2 = plot::PlotPoints::from_parametric_callback(
                |t| (end_angle.cos() * t, end_angle.sin() * t),
                0.0..=1.0,
                1000,
            );

            let l3 = plot::PlotPoints::from_parametric_callback(
                |t| (t.cos(), t.sin()),
                end_angle..=start_angle,
                1000,
            );

            let mut pp_vec = vec![];
            pp_vec.append(&mut l2.points().to_vec());
            pp_vec.append(&mut l3.points().to_vec());
            pp_vec.append(&mut l1.points().to_vec());

            let pp_vec = pp_vec
                .iter_mut()
                .map(|pp| plot::PlotPoint::new(round(pp.x, 7), round(pp.y, 7)))
                .collect();


            wedges.push(plot::Polygon::new(plot::PlotPoints::Owned(pp_vec)).name(d.0.to_string()))
        }

        Self { wedges }
    }
}

pub fn round(num: f64, decimals: u32) -> f64 {
    let precison = 10i32.pow(decimals) as f64;
    (num * precison).round() / precison

}
