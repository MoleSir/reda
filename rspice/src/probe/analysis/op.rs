use std::collections::HashMap;
use runit::{Current, Number, Voltage};

#[derive(Debug, Clone)]
pub struct OpAnalysis {
    pub nodes: HashMap<String, Voltage>,
    pub branches: HashMap<String, Current>,
    pub internal_parameters: HashMap<String, Number>,
}

impl OpAnalysis {
    pub fn get_node(&self, name: &str) -> Option<&Voltage> {
        self.nodes.get(&name.to_lowercase())
    }

    pub fn get_branch(&self, name: &str) -> Option<&Current> {
        self.branches.get(&name.to_lowercase())
    }

    pub fn get_internal(&self, name: &str) -> Option<&Number> {
        self.internal_parameters.get(&name.to_lowercase())
    }
}