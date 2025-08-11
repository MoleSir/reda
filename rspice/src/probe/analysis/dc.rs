use std::{collections::HashMap, path::Path};
use runit::{Current, CurrentUnit, Number, Unit, UnitNumber, Voltage, VoltageUnit};

use crate::probe::Drawer;

use super::AnalysisError;

#[derive(Debug, Clone)]
pub struct DcAnalysis<U> {
    pub sweep: Vec<UnitNumber<U>>, 
    pub nodes: HashMap<String, Vec<Voltage>>,
    pub branches: HashMap<String, Vec<Current>>,
    pub internal_parameters: HashMap<String, Vec<Number>>,
}

pub type DcVoltageAnalysis = DcAnalysis<VoltageUnit>;
pub type DcCurrentAnalysis = DcAnalysis<CurrentUnit>;

impl<U: Unit> DcAnalysis<U> {
    pub fn get_node(&self, name: &str) -> Option<&Vec<Voltage>> {
        self.nodes.get(name)
    }

    pub fn get_branch(&self, name: &str) -> Option<&Vec<Current>> {
        self.branches.get(name)
    }

    pub fn get_internal(&self, name: &str) -> Option<&Vec<Number>> {
        self.internal_parameters.get(name)
    }

    pub fn get_voltage_at(&self, node: &str, when: UnitNumber<U>) -> Option<Voltage> {
        let values = self.nodes.get(node)?;
        if values.len() != self.sweep.len() || values.len() < 2 {
            return None;
        }

        for i in 0..self.sweep.len() - 1 {
            let t0 = self.sweep[i];
            let t1 = self.sweep[i + 1];

            if when >= t0 && when <= t1 {
                let v0 = values[i];
                let v1 = values[i + 1];

                let ratio = (when - t0) / (t1 - t0);
                return Some(v0 + (v1 - v0) * ratio);
            }
        }

        None 
    }
}

impl<U: Unit> DcAnalysis<U> {
    pub fn draw_all_nodes<P: AsRef<Path>>(&self, drawer: &Drawer, path: P) -> Result<(), AnalysisError> {
        self.draw_nodes_filter(drawer, path, |_| true)
    }

    pub fn draw_nodes<P: AsRef<Path>>(
        &self, 
        drawer: &Drawer, 
        nodes: &[&str],
        path: P,
    ) -> Result<(), AnalysisError> {
        self.draw_nodes_filter(drawer, path, |name| nodes.contains(&name))
    }

    pub fn draw_nodes_filter<P: AsRef<Path>, Pre: Fn(&str) -> bool>(
        &self, 
        drawer: &Drawer, 
        path: P,
        predicate: Pre
    ) -> Result<(), AnalysisError> {
        let mut all_signals: Vec<(String, Vec<f64>)> = Vec::new();
        for (k, v) in &self.nodes {
            if predicate(k.as_str()) {
                let values = v.iter().map(|v| v.to_f64()).collect();
                all_signals.push((k.into(), values));
            }
        }
        let sweep: Vec<_> = self.sweep.iter().map(|t| t.to_f64()).collect();

        drawer.draw(U::name(), "V",  &sweep, &all_signals, path).map_err(|e| AnalysisError::PlotError(e))
    }

    pub fn draw_all_branchs<P: AsRef<Path>>(&self, drawer: &Drawer, path: P) -> Result<(), AnalysisError> {
        self.draw_branchs_filter(drawer, path, |_| true)
    }

    pub fn draw_branchs<P: AsRef<Path>>(
        &self, 
        drawer: &Drawer, 
        branchs: &[&str],
        path: P,
    ) -> Result<(), AnalysisError> {
        self.draw_branchs_filter(drawer, path, |name| branchs.contains(&name))
    }

    pub fn draw_branchs_filter<P: AsRef<Path>, Pre: Fn(&str) -> bool>(
        &self, 
        drawer: &Drawer, 
        path: P,
        predicate: Pre
    ) -> Result<(), AnalysisError> {
        let mut all_signals: Vec<(String, Vec<f64>)> = Vec::new();
        for (k, c) in &self.branches {
            if predicate(k.as_str()) {
                let values = c.iter().map(|v| v.to_f64()).collect();
                all_signals.push((k.into(), values));
            }
        }
        let sweep: Vec<_> = self.sweep.iter().map(|t| t.to_f64()).collect();

        drawer.draw(U::name(), "I",  &sweep, &all_signals, path).map_err(|e| AnalysisError::PlotError(e))
    }
}