mod error;
use std::path::Path;

use derive_builder::Builder;
pub use error::*;

use plotters::{
    chart::ChartBuilder, 
    prelude::{BitMapBackend, IntoDrawingArea, PathElement}, 
    series::LineSeries, 
    style::{Color, Palette, Palette99, RGBColor, BLACK, RED, WHITE}
};

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option, into))]
pub struct Drawer {
    #[builder(default = "false")]
    pub split: bool,

    #[builder(default = "1280")]
    pub width: u32,

    #[builder(default = "720")]
    pub height: u32,
    
    #[builder(default = "WHITE")]
    pub background_color: RGBColor,

    #[builder(default = "RED")]
    pub line_color: RGBColor,

    #[builder(default = "(\"sans-serif\", 15)")]
    pub font: (&'static str, u32),
}

impl Default for Drawer {
    fn default() -> Self {
        DrawerBuilder::default().build().unwrap()
    }
}

impl Drawer {
    pub fn draw<P: AsRef<Path>>(
        &self, 
        x_label: &str,
        y_label: &str,
        x: &[f64], 
        ys: &[(String, Vec<f64>)], 
        path: P
    ) -> Result<(), DrawerError> { 
        if self.split {
            self.draw_split(x_label, y_label, x, ys, path)
        } else {
            self.draw_combined(x_label, y_label, x, ys, path)
        }
    } 

    pub fn draw_split<P: AsRef<Path>>(
        &self, 
        x_label: &str,
        y_label: &str,
        x: &[f64], 
        ys: &[(String, Vec<f64>)], 
        path: P
    ) -> Result<(), DrawerError> {    
        let root = BitMapBackend::new(path.as_ref(), (self.width, self.height)).into_drawing_area();
        root
            .fill(&self.background_color)
            .map_err(|e| DrawerError::FillBackground(e.to_string()))?;
    
        let n = ys.len().max(1);
        let rows = n;
    
        let areas = root.split_evenly((rows, 1));
    
        for ((label, values), area) in ys.iter().zip(areas) {
            let (min_y, max_y) = values.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &v| {
                (min.min(v), max.max(v))
            });
    
            let mut chart = ChartBuilder::on(&area)
                .margin(5)
                .x_label_area_size(20)
                .y_label_area_size(40)
                .caption(label, self.font)
                .build_cartesian_2d(
                    x.first().copied().unwrap_or(0.0)..x.last().copied().unwrap_or(1.0),
                    min_y..max_y,
                )
                .map_err(|e| DrawerError::BuildCartesian(e.to_string()))?;
    
            chart
                .configure_mesh()
                .x_desc(x_label)
                .y_desc(y_label)
                .draw()
                .map_err(|e| DrawerError::DrawChart(e.to_string()))?;
    
            chart.draw_series(LineSeries::new(
                x.iter().cloned().zip(values.iter().cloned()),
                &self.line_color,
            ))
            .map_err(|e| DrawerError::DrawLine(label.clone(), e.to_string()))?;
        }
    
        Ok(())
    }

    pub fn draw_combined<P: AsRef<Path>>(
        &self,
        x_label: &str,
        y_label: &str,
        x: &[f64], 
        ys: &[(String, Vec<f64>)], 
        path: P,
    ) -> Result<(), DrawerError> {
        let root = BitMapBackend::new(path.as_ref(), (self.width, self.height)).into_drawing_area();
        root
            .fill(&self.background_color)
            .map_err(|e| DrawerError::FillBackground(e.to_string()))?;

        let (min_y, max_y) = ys
            .iter()
            .flat_map(|(_, v)| v.iter())
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &v| {
                (min.min(v), max.max(v))
            });

        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .caption("Combined Plot", self.font)
            .x_label_area_size(30)
            .y_label_area_size(50)
            .build_cartesian_2d(
                x.first().copied().unwrap_or(0.0)..x.last().copied().unwrap_or(1.0),
                min_y..max_y,
            )
            .map_err(|e| DrawerError::BuildCartesian(e.to_string()))?;

        chart
            .configure_mesh()
            .x_desc(x_label)
            .y_desc(y_label)
            .draw()
            .map_err(|e| DrawerError::DrawChart(e.to_string()))?;

        for (i, (label, values)) in ys.iter().enumerate() {
            let color = Palette99::pick(i).mix(0.9);
            chart
                .draw_series(LineSeries::new(
                    x.iter().cloned().zip(values.iter().cloned()),
                    &color,
                ))
                .map_err(|e| DrawerError::DrawLine(label.clone(), e.to_string()))?
                .label(label)
                .legend(move |(x, y)| {
                    PathElement::new([(x, y), (x + 20, y)], &color)
                });
        }

        chart
            .configure_series_labels()
            .border_style(&BLACK)
            .draw()
            .map_err(|e| DrawerError::DrawChart(e.to_string()))?;

        Ok(())
    }
}
