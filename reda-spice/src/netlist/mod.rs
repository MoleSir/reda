use crate::{simulate::{Simulate, Simulator}, Component, Instance, Model, Source, Subckt, ToSpice};
mod add;

#[derive(Debug, Default)]
pub struct Circuit {
    pub title: String,
    pub components: Vec<Component>,
    pub sources: Vec<Source>,
    pub subckts: Vec<Subckt>,
    pub instances: Vec<Instance>,
    pub models: Vec<Model>,
}

impl Circuit {
    pub fn new<S: Into<String>>(title: S) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    pub fn to_spice(&self) -> String {
        let mut lines = vec![];

        lines.push(format!(".title {}", self.title));
        for c in self.components.iter() {
            lines.push(c.to_spice());
        } 
        for s in self.sources.iter() {
            lines.push(s.to_spice());
        }
        for s in self.subckts.iter() {
            lines.push(s.to_spice());
        }
        for i in self.instances.iter() {
            lines.push(i.to_spice());
        }
        for m in self.models.iter() {
            lines.push(m.to_spice());
        }        

        lines.join("\n")
    }

    pub fn simulator<S: Simulate>(self, simulate: S) -> Simulator<S> {
        Simulator::<S>::new(self, simulate)
    }
}

#[allow(unused)]
#[cfg(test)]
mod test {
    use crate::{simulate::{ngspice::NgSpiceShared, Simulator}, DcCommand, DcCommandBuilder};
    use reda_unit::{num, u};

    use super::Circuit;

    #[test]
    fn test_dc() {
        /*
            * Simple DC Sweep
            V1 in 0 DC 0
            R1 in out 2k
            R2 out 0 1k
            .dc V1 0 5 0.1
            .end
        */
        let mut cir = Circuit::new("Simple DC Sweep");
        cir.add_dc_voltage("1", "in", "0", u!(0. V));
        cir.add_resistor("1", "in", "out", u!(2. kΩ));
        cir.add_resistor("2", "out", "0", u!(1. kΩ));
    
        let shared = NgSpiceShared::default().unwrap();
        let mut simulator = cir.simulator(shared);
        
        let command = DcCommandBuilder::default()
            .src_name("V1")
            .start(u!(0. V))
            .stop(u!(5. V))
            .step(u!(0.1 V))
            .build().unwrap();

        let analysis = simulator.run_dc_voltage(&command).expect("run dc");
        println!("{}", analysis.get_voltage_at("out", u!(200 uV)).unwrap());
    }
}