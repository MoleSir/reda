use crate::{
    netlist::Circuit, 
    probe::{AcAnalysis, DcVoltageAnalysis, OpAnalysis, TranAnalysis}, 
    AcCommand, DcCommand, ToSpice, TranCommand
};
pub mod ngspice;

pub trait Simulate {
    type Err;
    fn run_op(&mut self, netlist: &str) -> Result<OpAnalysis, Self::Err>;
    fn run_dc(&mut self, netlist: &str) -> Result<DcVoltageAnalysis, Self::Err>;
    fn run_tran(&mut self, netlist: &str) -> Result<TranAnalysis, Self::Err>;
    fn run_ac(&mut self, netlist: &str) -> Result<AcAnalysis, Self::Err>;
}

#[derive(Debug)]
pub struct Simulator<S> {
    pub circuit: Circuit,
    pub simulate: S,
}

impl<S: Simulate> Simulator<S> {
    pub fn new(circuit: Circuit, simulate: S) -> Self {
        Self { 
            circuit, 
            simulate,
        }
    }

    pub fn run_op(&mut self) -> Result<OpAnalysis, S::Err> {
        let mut circuit = self.circuit.to_spice();
        circuit.push_str("\n.op\n.end");
        self.simulate.run_op(&circuit)
    }

    pub fn run_tran(&mut self, command: &TranCommand) -> Result<TranAnalysis, S::Err> {
        let mut circuit = self.circuit.to_spice();
        circuit.push('\n');
        circuit.push_str(&command.to_spice());
        circuit.push_str("\n.end");
        self.simulate.run_tran(&circuit)
    }

    pub fn run_dc_voltage(&mut self, command: &DcCommand) -> Result<DcVoltageAnalysis, S::Err> {
        let mut circuit = self.circuit.to_spice();
        circuit.push('\n');
        circuit.push_str(&command.to_spice());
        circuit.push_str("\n.end");
        self.simulate.run_dc(&circuit)
    }

    pub fn run_ac(&mut self, command: &AcCommand) -> Result<AcAnalysis, S::Err> {
        let mut circuit = self.circuit.to_spice();
        circuit.push('\n');
        circuit.push_str(&command.to_spice());
        circuit.push_str("\n.end");
        self.simulate.run_ac(&circuit)        
    }
}

