use std::{collections::HashMap, path::Path};
use reda_unit::{Current, Number, Time, Voltage};

use crate::probe::Drawer;

use super::AnalysisError;

#[derive(Debug, Clone)]
pub struct TranAnalysis {
    pub time: Vec<Time>,
    pub nodes: HashMap<String, Vec<Voltage>>,
    pub branches: HashMap<String, Vec<Current>>,
    pub internal_parameters: HashMap<String, Vec<Number>>,
}

impl TranAnalysis {
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
        let time: Vec<_> = self.time.iter().map(|t| t.to_f64()).collect();

        drawer.draw("time", "V",  &time, &all_signals, path).map_err(|e| AnalysisError::PlotError(e))
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
        let time: Vec<_> = self.time.iter().map(|t| t.to_f64()).collect();

        drawer.draw("time", "I",  &time, &all_signals, path).map_err(|e| AnalysisError::PlotError(e))
    }
}

impl TranAnalysis {
    pub fn get_node(&self, name: &str) -> Option<&Vec<Voltage>> {
        self.nodes.get(name)
    }

    pub fn get_branch(&self, name: &str) -> Option<&Vec<Current>> {
        self.branches.get(name)
    }

    pub fn get_internal(&self, name: &str) -> Option<&Vec<Number>> {
        self.internal_parameters.get(name)
    }

    pub fn get_voltage_at(&self, node: &str, time: Time) -> Result<Voltage, AnalysisError> {
        let values = self.get_node(node)
            .ok_or_else(|| AnalysisError::NoExitNode(node.to_string()))?;

        if values.len() != self.time.len() || values.len() < 2 {
            return Err(AnalysisError::InnerError(format!("Bad value/time in tran analysis")));
        }

        // t[i] <= time_query <= t[i+1]
        let i = self.get_most_close_time(time) 
            .ok_or(AnalysisError::TimeOutOfRange(time))?;

        let t0 = self.time[i];
        let t1 = self.time[i + 1];
        let v0 = values[i];
        let v1 = values[i + 1];
        
        let ratio = (time - t0) / (t1 - t0);
        return Ok(v0 + (v1 - v0) * ratio);
    }

    pub fn get_current_at(&self, branch: &str, time: Time) -> Result<Current, AnalysisError> {
        let values = self.get_branch(branch)
            .ok_or_else(|| AnalysisError::NoExitBranch(branch.to_string()))?;

        if values.len() != self.time.len() || values.len() < 2 {
            return Err(AnalysisError::InnerError(format!("Bad value/time in tran analysis")));
        }

        // t[i] <= time_query <= t[i+1]
        let i = self.get_most_close_time(time) 
            .ok_or(AnalysisError::TimeOutOfRange(time))?;

        let t0 = self.time[i];
        let t1 = self.time[i + 1];
        let v0 = values[i];
        let v1 = values[i + 1];
        
        let ratio = (time - t0) / (t1 - t0);
        return Ok(v0 + (v1 - v0) * ratio);
    }
    
    fn get_most_close_time(&self, time: Time) -> Option<usize> {
        assert!(self.time.len() >= 2);
        for i in 0..self.time.len() - 1 {
            let t0 = self.time[i];
            let t1 = self.time[i + 1];

            if time >= t0 && time <= t1 {
                return Some(i);
            }
        }

        None 
    }
}