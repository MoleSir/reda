use std::{collections::HashMap, path::Path};
use runit::{Complex, CurrentPhasor, Frequency, VoltagePhasor};

use crate::probe::Drawer;

use super::AnalysisError;

#[derive(Debug, Clone, Default)]
pub struct AcAnalysis {
    pub frequency: Vec<Frequency>,
    pub nodes: HashMap<String, Vec<VoltagePhasor>>,
    pub branches: HashMap<String, Vec<CurrentPhasor>>,
    pub internal_parameters: HashMap<String, Vec<Complex>>,
}

impl AcAnalysis {
    pub fn get_node(&self, name: &str) -> Option<&Vec<VoltagePhasor>> {
        self.nodes.get(name)
    }

    pub fn get_branch(&self, name: &str) -> Option<&Vec<CurrentPhasor>> {
        self.branches.get(name)
    }    
}

impl AcAnalysis {
    pub fn draw_gain<P: AsRef<Path>>(
        &self, 
        drawer: &Drawer, 
        input_node: &str,
        output_node: &str,
        path: P
    ) -> Result<(), AnalysisError> {
        let frequency: Vec<_> = self.frequency.iter().map(|t| t.to_f64().log10()).collect();
        
        let input = self.get_node(input_node)
            .ok_or_else(|| AnalysisError::NoExitNode(input_node.into()))?;
        let output = self.get_node(output_node)
            .ok_or_else(|| AnalysisError::NoExitNode(output_node.into()))?;

        let mut values = Vec::new();
        for (vin, vout) in input.iter().zip(output.iter()) {
            let db = 20.0 * (vout.abs() / vin.abs()).log10();
            values.push(db.to_f64());
        }

        drawer.draw("frequency", "Gain",  &frequency, &[("Gain".into(), values)], path).map_err(|e| AnalysisError::PlotError(e))
    }

    pub fn draw_phase<P: AsRef<Path>>(
        &self, 
        drawer: &Drawer, 
        input_node: &str,
        output_node: &str,
        path: P
    ) -> Result<(), AnalysisError> {
        let frequency: Vec<_> = self.frequency.iter().map(|t| t.to_f64().log10()).collect();
        
        let input = self.get_node(input_node)
            .ok_or_else(|| AnalysisError::NoExitNode(input_node.into()))?;
        let output = self.get_node(output_node)
            .ok_or_else(|| AnalysisError::NoExitNode(output_node.into()))?;

        let mut values = Vec::new();
        for (vin, vout) in input.iter().zip(output.iter()) {
            // let db = 20.0 * (vout.abs() / vin.abs()).log10();
            let relative = vout.arg() - vin.arg();
            values.push(relative.to_f64());
        }

        drawer.draw("frequency", "Phase",  &frequency, &[("Phase".into(), values)], path).map_err(|e| AnalysisError::PlotError(e))
    }
}