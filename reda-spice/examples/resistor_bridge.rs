use reda_spice::{netlist::Circuit, simulate::ngspice::NgSpiceShared};
use reda_unit::u;

/// https://pyspice.fabrice-salvaire.fr/releases/v1.4/examples/resistor/resistor-bridge.html
fn main_result() -> Result<(), Box<dyn std::error::Error>> {
    let mut circuit = Circuit::new("Resistor Bridge");

    circuit.add_dc_voltage("input", "1", "0", u!(10 V));
    circuit.add_resistor("1", "1", "2", u!(2 kΩ));
    circuit.add_resistor("2", "1", "3", u!(1 kΩ));
    circuit.add_resistor("3", "2", "0", u!(1 kΩ));
    circuit.add_resistor("4", "3", "0", u!(2 kΩ));
    circuit.add_resistor("5", "3", "2", u!(2 kΩ));

    let simulate = NgSpiceShared::default()?;
    let mut simulator = circuit.simulator(simulate);
    let analysis = simulator.run_op()?;

    for (name, node) in analysis.nodes.iter() {
        println!("Node {}: {}", name, node)
    }

    Ok(())
}

fn main() {
    if let Err(e) = main_result() {
        eprintln!("{}", e);
    }
}